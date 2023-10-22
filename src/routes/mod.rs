use axum::{Router, http::{Method, Request}, middleware::{Next, self}, response::Response};
use chrono::Local;
use sqlx::SqlitePool;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};

mod saves;
mod files;
mod note_categories;
mod notes;

/// Initializes the server's router
pub fn get_router(pool: SqlitePool) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH, Method::OPTIONS])
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let saves_router = saves::get_router(pool.clone());
    let files_router = files::get_router(pool.clone());
    let note_categories_router = note_categories::get_router(pool.clone());
    let notes_router = notes::get_router(pool.clone());

    Router::new()
        .merge(saves_router)
        .merge(files_router)
        .merge(note_categories_router)
        .merge(notes_router)
        .layer(cors)
        .route_layer(middleware::from_fn(route_logging))
}

/// Middleware between all requests. Just prints some request info
async fn route_logging<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("\n{}:\n{} {}", now, req.method(), req.uri());

    next.run(req).await
}
