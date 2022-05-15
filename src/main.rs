use std::env::{var_os, set_var};
use axum::Router;
use rs_blog::handler::{client};

#[tokio::main]
async fn main() {
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "rs_blog=debug");
    }

    tracing_subscriber::fmt::init();
    tracing::info!("server started at http://localhost:3000 !");

    let app = Router::new()
        .nest("/", client::router());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
