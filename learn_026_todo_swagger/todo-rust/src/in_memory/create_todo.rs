use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::domain::newtodo::NewTodo;
use crate::domain::state::AppState;
use crate::domain::todo::Todo;

#[utoipa::path(post, path = "/todos")]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(todo): Json<NewTodo>,
) -> Json<Vec<Todo>> {
    let mut todos = state.todos.write().expect("mutex poisoned");
    let todo = Todo {
        id: state.get_id(),
        title: todo.title,
        completed: false,
    };
    todos.push(todo);
    Json(todos.clone())
}

pub type DynCreateRepos = Arc<dyn CreateRepos + Send + Sync>;

pub trait CreateRepos {
    async fn create_todo(&self, state: &State<AppState>, todo: &Json<NewTodo>) -> Json<Vec<Todo>>;
}
