use crate::prisma::{task, PrismaClient};
use axum::Extension;
use prisma_client_rust::prisma_errors::query_engine::{RecordNotFound, RecordRequiredButNotFound};
use prisma_client_rust::QueryError;
use std::sync::Arc;

type Database = Extension<Arc<PrismaClient>>;
pub trait TaskDao {
    async fn query_tasks(&self) -> Result<Vec<task::Data>, DBError>;
    async fn create_task(&self, description: String) -> Result<task::Data, DBError>;
    async fn complete_task(&self, id: String) -> Result<task::Data, DBError>;
    async fn delete_task(&self, id: String) -> Result<task::Data, DBError>;
}

impl TaskDao for Database {
    async fn query_tasks(&self) -> Result<Vec<task::Data>, DBError> {
        self.task()
            .find_many(vec![])
            .exec()
            .await
            .map_err(DBError::from) // can't get rid of ::from cause of alias return type of the exec() function
    }

    async fn create_task(&self, description: String) -> Result<task::Data, DBError> {
        self.task()
            .create(description, vec![])
            .exec()
            .await
            .map_err(DBError::from)
    }

    async fn complete_task(&self, id: String) -> Result<task::Data, DBError> {
        self.task()
            .update(
                task::id::equals(id.clone()),
                vec![task::completed::set(true)],
            )
            .exec()
            .await
            .map_err(DBError::from)
    }

    async fn delete_task(&self, id: String) -> Result<task::Data, DBError> {
        self.task()
            .delete(task::id::equals(id.clone()))
            .exec()
            .await
            .map_err(DBError::from)
    }
}
#[derive(Debug)]
pub enum DBError {
    NotFound,
    Other(QueryError),
}
impl From<QueryError> for DBError {
    fn from(query_error: QueryError) -> Self {
        match query_error {
            e if e.is_prisma_error::<RecordNotFound>() => DBError::NotFound,
            e if e.is_prisma_error::<RecordRequiredButNotFound>() => DBError::NotFound,
            e => DBError::Other(e),
        }
    }
}
