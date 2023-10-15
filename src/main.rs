use axum::{Router, response::{IntoResponse, Response}, routing::get, http::{StatusCode, Request}, Json, middleware::{self, Next}};
use chrono::Local;
use std::net::SocketAddr;

mod db;
mod models;
mod routes;

use models::res::ServerResult;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().unwrap();

    // Setting up the db
    let pool = db::get_pool().await;
    db::reset(&pool).await;

    // Setting up the routes
    let saves_router = routes::saves::get_router(pool);

    let app = Router::new()
        .route("/", get(root))
        .merge(saves_router)
        .route_layer(middleware::from_fn(mw));

    // Setting up the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 7272));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, res_body!(true, Some("hello there"), Some(())))
}

pub async fn mw<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("\n{}:\n{} {}", now, req.method(), req.uri());

    next.run(req).await
}
