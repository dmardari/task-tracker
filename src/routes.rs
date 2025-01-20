use crate::prisma::{task, PrismaClient};
use axum::extract::Path;
use axum::routing::{delete, get, patch, post};
use axum::{Extension, Json, Router};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
type Database = Extension<Arc<PrismaClient>>;

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(list_tasks))
        .route("/", post(create_task))
        .route("/{id}/complete", patch(complete_task))
        .route("/{id}", delete(delete_task))
}

async fn list_tasks(db: Database) -> Result<Json<Vec<task::Data>>, (StatusCode, String)> {
    let tasks = match db.task().find_many(vec![]).exec().await {
        Ok(data) => data,
        Err(e) => {
            println!("Unable to query tasks: {e}");
            todo!("handle the error");
        }
    };
    Ok(Json::from(tasks))
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

#[derive(Deserialize)]
struct CreateTask {
    description: String,
}

#[derive(Serialize)]
struct Task {
    id: u64,
    description: String,
}
