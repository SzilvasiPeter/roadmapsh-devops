API: axum
DB: surrealdb

curl -s -X POST http://127.0.0.1:3000/todos -H "Content-Type: application/json" -d '{"title":"test task2","completed":false}'
curl http://127.0.0.1:3000/todos
