use axum::{
    body::Body,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks", get(list_tasks));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn list_tasks() -> impl IntoResponse {
    let tasks = [Task {
        id: 1337,
        tasktitle: "Test title".to_string(),
    }];

    (StatusCode::OK, Json(tasks))
}

async fn get_task(Path(id): Path<i32>) -> impl IntoResponse {
    if id == 42 {
        let task = Task {
            id: 42,
            tasktitle: "fourty-two".to_string(),
        };
        (StatusCode::OK, Json(task).into_response())
    } else {
        (StatusCode::NOT_FOUND, Body::empty().into_response())
    }
}

async fn create_task(Json(payload): Json<CreateTask>) -> impl IntoResponse {
    let _task = Task {
        id: 1337,
        tasktitle: payload.tasktitle,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Body::empty())
    // (StatusCode::CREATED, Json(task))
}

// the input to our `create_task` handler
#[derive(Deserialize)]
struct CreateTask {
    tasktitle: String,
}

// the output to our `create_task` handler
#[derive(Serialize)]
struct Task {
    id: u64,
    tasktitle: String,
}
