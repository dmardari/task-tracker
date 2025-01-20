use crate::prisma::PrismaClient;
use axum::Router;
use prisma_client_rust::NewClientError;
#[allow(warnings)]
mod prisma;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let prisma_client: Result<PrismaClient, NewClientError> =
        PrismaClient::_builder().build().await;

    let app = Router::new().nest("/tasks", routes::create_route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("A listener is expected to be bound");
    axum::serve(listener, app)
        .await
        .expect("Axum server should be successfully started");
}
