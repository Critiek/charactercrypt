use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub wordlist: Arc<Vec<String>>,
}
