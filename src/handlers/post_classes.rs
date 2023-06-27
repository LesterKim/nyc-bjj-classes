use crate::database::AppState;
use crate::models::Class;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::{Arc, Mutex};

pub async fn post_classes(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(class): Json<Class>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();

    state.database.add_class(class);

    StatusCode::CREATED
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::{AppState, Database};
    use axum::extract::{Json, State};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_post_class() {
        let database = Database::Mock(vec![]);
        let app_state = Arc::new(Mutex::new(AppState { database }));
        let state = State(app_state.clone());
        let class = Class::new(
            1,
            1,
            1,
            "Monday".into(),
            "18:00".into(),
            "Manhattan".into(),
            30.0,
        );

        let json = Json(class.clone());
        let _ = post_classes(state, json).await;

        let classes = app_state.lock().unwrap().database.get_classes();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0], class);
    }
}
