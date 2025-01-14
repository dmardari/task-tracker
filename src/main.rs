use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum::extract::Path;
use axum::routing::{delete, patch};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /tasks` goes to `dummy`
        .route("/tasks", get(dummy))
        // `POST /tasks` goes to `create_task`
        .route("/tasks", post(create_task))
        // `PATCH /tasks/{id}/complete` goes to `complete_task`
        .route("/tasks/{id}/complete", patch(complete_task))
        // `DELETE /tasks/{id}` goes to `delete_task`
        .route("/tasks/{id}", delete(delete_task));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn dummy() -> (StatusCode, Json<String>) {
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
async fn create_task(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateTask` type
    Json(payload): Json<CreateTask>,
) -> (StatusCode, Json<Task>) {
    // insert your application logic here
    let task = Task {
        id: 1337,
        description: payload.description,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(task))
}

async fn complete_task(Path(_id): Path<u32>) -> (StatusCode, Json<String>) {
    let message = "Not Yet Implemented";

    (StatusCode::NOT_IMPLEMENTED, Json(message.to_owned()))
}

async fn delete_task() -> (StatusCode, Json<String>) {
    let message = "Not Yet Implemented";

    (StatusCode::NOT_IMPLEMENTED, Json(message.to_owned()))
}
