#![recursion_limit = "512"]

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use demo_index::IndexPage;
use elementary_rs_lib::page::render_page;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .nest_service("/wasm", ServeDir::new("target-wasm/pkg"));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    let page = render_page(IndexPage { x: 20 }).await.unwrap();
    Html(page).into_response()
}
