use axum::Router;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().nest("/tasks", routes::create_route());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("A listener is expected to be bound");
    axum::serve(listener, app)
        .await
        .expect("Axum server should be successfully started");
}
