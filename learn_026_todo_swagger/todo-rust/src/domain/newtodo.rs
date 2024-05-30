use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub(crate) struct NewTodo {
    pub title: String,
}
