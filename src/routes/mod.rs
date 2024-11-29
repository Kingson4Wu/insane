use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers;
use crate::state::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/visit", get(handlers::increment_counter))
        .route("/echo", post(handlers::echo_message))
        .with_state(state)
}
