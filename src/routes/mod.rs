use axum::{Router, http::{Method, Request}, middleware::{Next, self}, response::Response};
use chrono::Local;
use sqlx::SqlitePool;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};

mod saves;

/// Initializes the server's router
pub fn get_router(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH, Method::OPTIONS])
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let saves_router = saves::get_router(pool);

    Router::new()
        .merge(saves_router)
        .layer(cors)
        .route_layer(middleware::from_fn(mw))
}

/// Middleware between all requests. Just prints some request info
async fn mw<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("\n{}:\n{} {}", now, req.method(), req.uri());

    next.run(req).await
}
