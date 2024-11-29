mod handlers;
mod models;
mod routes;
mod state;

use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create application state
    let state = state::AppState::new();

    // Create router
    let app = routes::create_router(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}
