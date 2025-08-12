pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;

use axum::{
    extract::Request, extract::State, http::StatusCode, middleware::Next, response::Response,
};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use serde::Deserialize;
use std::process;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub port: i16,
    pub db_path: String,
    pub token: String,
}

pub fn read_config_file(path: &str) -> ServerConfig {
    let config_file = std::fs::read_to_string(path).expect("Failed to read config file.");
    let config: ServerConfig =
        serde_json::from_str(&config_file).expect("Failed to parse config file");

    if config.token.len() < 20 {
        println!("The token should have a length of at least 20");
        process::exit(1);
    }

    config
}

pub fn establish_connection(db_path: &str) -> SqliteConnection {
    SqliteConnection::establish(db_path).unwrap_or_else(|e| panic!("Failed to connect, error: {e}"))
}

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub fn run_migrations(
    db_path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut connection = establish_connection(db_path);
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

/// Enforce that a Bearer token is set in the header of every request
pub async fn enforce_correct_bearer_token(
    State(config): State<ServerConfig>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // use tracing::debug;
    // debug!("{:?}", req.headers());

    let api_token_received = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?
        .split_whitespace()
        .nth(1)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if api_token_received == config.token {
        // If the API key matches, proceed to the next handler
        Ok(next.run(req).await)
    } else {
        // Otherwise, return Unauthorized
        Err(StatusCode::UNAUTHORIZED)
    }
}
