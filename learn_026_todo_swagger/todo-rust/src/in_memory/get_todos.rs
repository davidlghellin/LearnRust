use axum::extract::State;
use axum::Json;

use crate::domain::state::AppState;
use crate::domain::todo::Todo;

#[utoipa::path(get, path = "/todos")]
pub async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    log::info!("log de get todos");
    log::debug!("state: {:?}",&state);
    Json(state.todos.read().expect("mutex poisoned").clone())
}
