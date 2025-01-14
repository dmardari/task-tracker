use axum::extract::Path;
use axum::routing::{delete, patch};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/tasks", get(list_tasks))
        .route("/tasks", post(create_task))
        .route("/tasks/{id}/complete", patch(complete_task))
        .route("/tasks/{id}", delete(delete_task));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("A listener is expected to be bound");
    axum::serve(listener, app)
        .await
        .expect("Axum server should be successfully started");
}

async fn list_tasks() -> (StatusCode, Json<String>) {
    let message = "Not Yet Implemented";
    (StatusCode::NOT_IMPLEMENTED, Json(message.to_owned()))
}

#[derive(Deserialize)]
struct CreateTask {
    description: String,
}

#[derive(Serialize)]
struct Task {
    id: u64,
    description: String,
}
async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let task = Task {
        id: 1337,
        description: payload.description,
    };
    (StatusCode::CREATED, Json(task))
}

async fn complete_task(Path(_id): Path<u32>) -> (StatusCode, Json<String>) {
    let message = "Not Yet Implemented";
    (StatusCode::NOT_IMPLEMENTED, Json(message.to_owned()))
}

async fn delete_task(Path(_id): Path<u32>) -> (StatusCode, Json<String>) {
    let message = "Not Yet Implemented";
    (StatusCode::NOT_IMPLEMENTED, Json(message.to_owned()))
}
