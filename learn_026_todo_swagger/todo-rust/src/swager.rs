use utoipa::OpenApi;

use crate::domain::todo::Todo;
use crate::domain::newtodo::NewTodo;
use crate::in_memory::get_todos::__path_get_todos;
use crate::in_memory::delete_todo::__path_delete_todo;
use crate::in_memory::create_todo::__path_create_todo;

#[derive(OpenApi)]
#[openapi(
    paths(get_todos,create_todo,delete_todo),
    components(schemas(Todo,NewTodo)),
    tags((name = "Todo App", description = "Mini app de todos"))
)]
pub struct ApiDoc;
