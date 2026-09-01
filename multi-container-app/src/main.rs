use axum::{Json, Router, extract::State, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;
use surrealdb::types::{RecordId, SurrealValue};

#[derive(Debug, Serialize, Deserialize, SurrealValue)]
struct Todo {
    id: Option<RecordId>,
    title: String,
    completed: bool,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<Client>>,
}

async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos: Vec<Todo> = state
        .db
        .query("SELECT * FROM todos")
        .await
        .expect("Failed to query")
        .take(0)
        .expect("Failed to get todos");
    Json(todos)
}

async fn create_todo(
    State(state): State<AppState>,
    Json(todo): Json<Todo>,
) -> Json<Todo> {
    let created: Option<Todo> = state
        .db
        .create("todos")
        .content(todo)
        .await
        .expect("Failed to create todo")
        .expect("Failed to insert");

    Json(created.expect("Failed to create todo"))
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("SURREALDB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string());

    let db = Surreal::new::<Ws>(&db_url)
        .await
        .expect("Failed to connect to SurrealDB");

    db.use_ns("my_namespace")
        .use_db("my_database")
        .await
        .expect("Failed to use namespace/database");

    let state = AppState {
        db: Arc::new(db),
    };

    let app = Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .with_state(state);

    let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap();

    println!("Server running on http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
