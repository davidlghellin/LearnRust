use axum::{
    Router
    , routing::{get, patch},
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::domain::state::AppState;
use crate::in_memory::create_todo::create_todo;
use crate::in_memory::delete_todo::delete_todo;
use crate::in_memory::get_todos::get_todos;
use crate::in_memory::update_todo::update_todo;
use crate::swager::ApiDoc;

pub(crate) fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", patch(update_todo).delete(delete_todo))
        .merge(SwaggerUi::new("/swagegr-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
}
