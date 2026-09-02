use axum::{Json, Router, extract::State, routing::get};
use meilisearch_sdk::client::Client;
use meilisearch_sdk::search::SearchResults;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

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

async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let results: SearchResults<Todo> = state
        .db
        .index("todos")
        .search()
        .with_query("")
        .execute()
        .await
        .expect("Failed to search todos");

    let todos: Vec<Todo> = results.hits.into_iter().map(|h| h.result).collect();
    Json(todos)
}

async fn create_todo(State(state): State<AppState>, Json(mut todo): Json<Todo>) -> Json<Todo> {
    todo.id = Uuid::new_v4().to_string();

    state
        .db
        .index("todos")
        .add_documents(&[&todo], Some("id"))
        .await
        .expect("Failed to add todo");

    Json(todo)
}

#[tokio::main]
async fn main() {
    let db_url =
        std::env::var("MEILISEARCH_URL").unwrap_or_else(|_| "http://127.0.0.1:7700".to_string());
    let db_key = std::env::var("MEILISEARCH_KEY").ok();

    let client = Client::new(&db_url, db_key).expect("Failed to connect to Meilisearch");

    let state = AppState {
        db: Arc::new(client),
    };

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .with_state(state);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Server running on http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
