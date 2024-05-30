use axum::extract::{Path, State};
use axum::Json;

use crate::domain::state::AppState;
use crate::domain::todo::Todo;

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(todo): Json<Todo>,
) -> Json<Vec<Todo>> {
    let mut todos = state.todos.write().expect("mutex poisoned");

    if let Some(index) = todos.iter().position(|t| t.id == id) {
        todos[index].completed = todo.completed;
        todos[index].title = todo.title.clone();
    }

    Json(todos.clone())
}
