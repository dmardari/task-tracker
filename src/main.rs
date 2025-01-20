use crate::prisma::PrismaClient;
use axum::{Extension, Router};
use std::sync::Arc;
#[allow(warnings)]
mod prisma;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let prisma_client: PrismaClient = PrismaClient::_builder()
        .build()
        .await
        .expect("A new DB connection should be established");

    let app = Router::new()
        .nest("/tasks", routes::create_route())
        .layer(Extension(Arc::new(prisma_client)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("A listener is expected to be bound");
    axum::serve(listener, app)
        .await
        .expect("Axum server should be successfully started");
}
