pub mod post_classes;

use crate::database::AppState;
use crate::models::Class;

use axum::{extract::State, Json};
use std::sync::{Arc, Mutex};

pub async fn get_classes(state: State<Arc<Mutex<AppState>>>) -> Json<Vec<Class>> {
    let state = state.lock().unwrap();
    let classes = state.database.get_classes();

    Json(classes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{AppState, Database};
    use axum::extract::State;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_get_classes() {
        let database = Database::Mock(vec![]);
        let state = State(Arc::new(Mutex::new(AppState { database })));
        let response = get_classes(state).await;
        assert!(response.0.is_empty());
    }
}
