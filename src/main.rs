use diesel::pg::PgConnection;
use dotenv::dotenv;
use routes::create_router;
use state::AppState;
use std::{env, sync::Arc};

extern crate diesel;
extern crate tracing;

mod errors;
mod models;
mod operations;
mod routes;
mod schema;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");

    let pool = diesel::r2d2::Pool::builder()
        .build(diesel::r2d2::ConnectionManager::<PgConnection>::new(
            database_url,
        ))
        .expect("Failed to create database connection pool");

    let app_state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_router(app_state);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
