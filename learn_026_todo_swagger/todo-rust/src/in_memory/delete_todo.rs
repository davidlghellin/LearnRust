use axum::extract::{Path, State};
use axum::Json;

use crate::domain::state::AppState;
use crate::domain::todo::Todo;

#[utoipa::path(delete, path = "/todos/{id}")]
pub async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> Json<Vec<Todo>> {
    let mut todos = state.todos.write().expect("mutex poisoned");
    if let Some(index) = todos.iter().position(|t| t.id == id) {
        todos.remove(index);
    }
    Json(todos.clone())
}
