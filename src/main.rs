use std::net::SocketAddr;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::{
    config::email_config::init_email_config,
    handlers::{
        email_handler::send_email_handler, index_handler::index_handler, pdf_handler::pdf_handler,
    },
    middlewares::my_middleware::custom_fn_middleware,
};

mod api_response;
mod config;
mod email;
mod error_response;
mod handlers;
mod middlewares;
mod times;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // load ENV
    dotenv().ok();
    init_email_config().await;

    // middleware
    let layer_middleware = middleware::from_fn(custom_fn_middleware);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/", post(index_handler))
        .route("/pdf", post(pdf_handler))
        .route("/send_email", post(send_email_handler))
        .layer(layer_middleware);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("listening on http://{}", addr);

    // timing
    // times::my_time::get_local_time();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("failed to start server");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
