mod db;
mod error;
mod handlers;
mod message;
mod models;
mod routes;
mod solana;
mod tracer;

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    dotenvy::dotenv().ok();
    let _guard = tracer::setup_tracing();

    let pool = db::init().await?;
    let app = routes::create_router(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
