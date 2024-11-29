use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

#[derive(Clone)]
pub struct AppState {
    pub visit_count: Arc<AtomicUsize>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            visit_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}
