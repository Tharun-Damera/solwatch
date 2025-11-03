use anyhow::Result;

mod handlers;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let app = routes::create_router();
    let listener = tokio::net::TcpListener::bind("localhost:5000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
