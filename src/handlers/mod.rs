use axum::{
    extract::State,
    response::Json,
};
use std::sync::atomic::Ordering;
use tracing::info;

use crate::models::{HealthCheck, Message};
use crate::state::AppState;

pub async fn health_check() -> Json<HealthCheck> {
    Json(HealthCheck {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn increment_counter(State(state): State<AppState>) -> String {
    let count = state.visit_count.fetch_add(1, Ordering::SeqCst);
    format!("Visit count: {}", count + 1)
}

pub async fn echo_message(Json(message): Json<Message>) -> Json<Message> {
    info!("Received message: {:?}", message);
    Json(message)
}
