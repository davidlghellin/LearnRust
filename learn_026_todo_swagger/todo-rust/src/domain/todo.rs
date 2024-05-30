use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
