use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

// Task struct
#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: Uuid,
    text: String,
}

// Shared state
type SharedState = Arc<Mutex<Vec<Task>>>;

// Handlers
async fn get_tasks(State(state): State<SharedState>) -> Json<Vec<Task>> {
    println!("Getting tasks");
    let tasks = state.lock().unwrap();
    Json(tasks.clone())
}

async fn add_task(
    State(state): State<SharedState>,
    Json(payload): Json<TaskRequest>,
) -> (StatusCode, Json<Task>) {
    println!("Adding task: {}", payload.text);
    let mut tasks = state.lock().unwrap();
    let new_task = Task {
        id: Uuid::new_v4(),
        text: payload.text,
    };
    tasks.push(new_task.clone());
    (StatusCode::CREATED, Json(new_task))
}

async fn delete_task(State(state): State<SharedState>, Path(id): Path<Uuid>) -> StatusCode {
    let mut tasks = state.lock().unwrap();
    if let Some(pos) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(pos);
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

// Task request struct
#[derive(Deserialize)]
struct TaskRequest {
    text: String,
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state: SharedState = Arc::new(Mutex::new(Vec::new()));

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins ---> temporal
        .allow_methods(Any) // Allow all methods (GET, POST, DELETE, etc.)
        .allow_headers(Any); // Allow all headers

    // Create the router
    let app = Router::new()
        .route("/tasks", get(get_tasks).post(add_task))
        .route("/tasks/:id", delete(delete_task))
        .with_state(state)
        .layer(cors); // Add CORS as middleware

    // Start the server
    println!("Listening on http://localhost:8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
