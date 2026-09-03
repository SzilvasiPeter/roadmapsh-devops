use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::{Error, ErrorCode};
use meilisearch_sdk::search::SearchResults;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Meilisearch error: {0}")]
    Meili(#[from] Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("{self:?}");

        match &self {
            Self::Meili(Error::Meilisearch(err))
                if err.error_code == ErrorCode::DocumentNotFound =>
            {
                (StatusCode::NOT_FOUND, "Document not found").into_response()
            }
            Self::Meili(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(default)]
    id: String,
    title: String,
    completed: bool,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Client>,
}

async fn get_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = state.db.index("todos");
    let results: SearchResults<Todo> = todos.search().with_query("").execute().await?;
    let todos: Vec<Todo> = results.hits.into_iter().map(|h| h.result).collect();

    Ok(Json(todos))
}

async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Todo>, AppError> {
    let todos = state.db.index("todos");
    let result: Todo = todos.get_document(&id).await?;

    Ok(Json(result))
}

async fn create_todo(
    State(state): State<AppState>,
    Json(mut todo): Json<Todo>,
) -> Result<Json<Todo>, AppError> {
    let todos = state.db.index("todos");

    todo.id = Uuid::new_v4().to_string();
    todos.add_or_replace(&[&todo], Some("id")).await?;

    Ok(Json(todo))
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(mut todo): Json<Todo>,
) -> Result<Json<Todo>, AppError> {
    let todos = state.db.index("todos");

    todo.id = id;
    todos.add_or_update(&[&todo], Some("id")).await?;

    Ok(Json(todo))
}

async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let todos = state.db.index("todos");

    todos
        .delete_document(&id)
        .await?
        .wait_for_completion(&state.db, None, None)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[tokio::main]
async fn main() {
    let default_url = "http://127.0.0.1:7700".to_string();
    let db_url = std::env::var("MEILISEARCH_URL").unwrap_or(default_url);
    let db_key = std::env::var("MEILISEARCH_KEY").ok();

    let client = Client::new(&db_url, db_key).expect("Failed to connect to Meilisearch");

    let db = Arc::new(client);
    let state = AppState { db };

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(state);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Server running on http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
