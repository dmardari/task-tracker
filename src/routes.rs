use crate::db::*;
use crate::prisma::{task, PrismaClient};
use axum::extract::Path;
use axum::routing::{delete, get, patch, post};
use axum::{Extension, Json, Router};
use http::StatusCode;
use serde::Deserialize;
use std::sync::Arc;

type Database = Extension<Arc<PrismaClient>>;

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(list_tasks))
        .route("/", post(create_task))
        .route("/{id}/complete", patch(complete_task))
        .route("/{id}", delete(delete_task))
}

async fn list_tasks(db: Database) -> Result<Json<Vec<task::Data>>, StatusCode> {
    let tasks = db.query_tasks().await?;
    Ok(Json::from(tasks))
}

async fn create_task(
    db: Database,
    Json(payload): Json<CreateTask>,
) -> Result<Json<task::Data>, StatusCode> {
    let task = db.create_task(payload.description).await?;
    Ok(Json::from(task))
}

async fn complete_task(
    db: Database,
    Path(id): Path<String>,
) -> Result<Json<task::Data>, StatusCode> {
    let task = db.complete_task(id).await?;
    Ok(Json::from(task))
}

async fn delete_task(db: Database, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    db.delete_task(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct CreateTask {
    description: String,
}

impl From<DBError> for StatusCode {
    fn from(e: DBError) -> Self {
        match e {
            DBError::NotFound => StatusCode::NOT_FOUND,
            DBError::Other(_) => StatusCode::BAD_REQUEST,
        }
    }
}
