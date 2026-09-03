# Multi-container App

A todo list API with full CRUD over `/todos` (list, create, get, update, and
delete todos), built with [Axum](https://github.com/tokio-rs/axum) and
[Meilisearch](https://github.com/meilisearch/meilisearch) as its data store.

## Testing

Build and start the containers (wait for Meilisearch healthcheck):

```sh
podman-compose up --build
```

Create a todo (`id` is generated server-side, no need to send it):

```sh
curl -s -X POST http://127.0.0.1:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"test task2","completed":false}'
```

List all todos:

```sh
curl http://127.0.0.1:3000/todos
```

Get a single todo by ID:

```sh
curl http://127.0.0.1:3000/todos/<id>
```

Update a todo by ID:

```sh
curl -s -X PUT http://127.0.0.1:3000/todos/<id> \
  -H "Content-Type: application/json" \
  -d '{"title":"updated task","completed":true}'
```

Delete a todo by ID:

```sh
curl -s -X DELETE http://127.0.0.1:3000/todos/<id>
```

Stop and remove the containers (and data volume):

```sh
podman-compose down --volumes
```
