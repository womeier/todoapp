use axum::middleware;
use backend::handlers::*;
use backend::*;
use clap::Parser;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::process;
use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,

    #[arg(short, long)]
    openapi_output_path: Option<String>,
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    security(
        ("bearerTokenAuth" = [])
    ),
    info(
        contact(
            name = "Wolfgang Meier",
            email = "womeier@posteo.de",
        ),
        license(
            name = "MIT",
            identifier = "MIT",
        )
    ),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerTokenAuth",
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = read_config_file(&args.config);

    run_migrations(&config.db_path).expect("Failed to apply sqlite migrations.");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let task_router = OpenApiRouter::new()
        .routes(routes!(get_task))
        .routes(routes!(list_tasks))
        .routes(routes!(create_task))
        .routes(routes!(update_task))
        .with_state(config.clone());

    // build our application with a route
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/", task_router)
        .split_for_parts();

    let app = router.route_layer(middleware::from_fn_with_state(
        config.clone(),
        enforce_correct_bearer_token,
    ));

    // Extract openapi schema if requested
    if let Some(path) = args.openapi_output_path {
        let api = api.to_pretty_json().unwrap();
        let output_file = File::create(&path).unwrap();
        let mut output_writer = BufWriter::new(output_file);
        let _ = output_writer.write(api.as_bytes());
        let _ = output_writer.flush();

        println!("Wrote openapi config file to {path}.");
        process::exit(0);
    }

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();

    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
