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

async fn create_task(
    db: Database,
    Json(payload): Json<CreateTask>,
) -> Result<Json<task::Data>, (StatusCode, String)> {
    let task = match db.task().create(payload.description, vec![]).exec().await {
        Ok(data) => data,
        Err(e) => {
            println!("Unable to create task: {e}");
            todo!("handle the error");
        }
    };
    Ok(Json::from(task))
}

async fn complete_task(
    db: Database,
    Path(id): Path<String>,
) -> Result<Json<task::Data>, (StatusCode, String)> {
    let task = match db
        .task()
        .update(
            task::id::equals(id.clone()),
            vec![task::completed::set(true)],
        )
        .exec()
        .await
    {
        Ok(data) => data,
        Err(e) => {
            println!("Unable to update the task {id}: {e}");
            todo!("handle the error");
        }
    };
    Ok(Json::from(task))
}

async fn delete_task(db: Database, Path(id): Path<String>) -> StatusCode {
    let task = match db.task().delete(task::id::equals(id.clone())).exec().await {
        Ok(data) => data,
        Err(e) => {
            println!("Unable to delete the task {id}: {e}");
            todo!("handle the error");
        }
    };
    StatusCode::NO_CONTENT
}

#[derive(Deserialize)]
struct CreateTask {
    description: String,
}
