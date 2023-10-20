use std::net::SocketAddr;

mod db;
mod models;
mod routes;
mod error;

#[tokio::main]
async fn main() {

    // Loading the .env
    dotenvy::dotenv().unwrap();

    // Setting up the db
    let pool = db::get_pool().await;
    // db::_reset(&pool).await;

    // Setting up the routes
    let app = routes::get_router(pool);

    // Setting up the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 7272));
    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
