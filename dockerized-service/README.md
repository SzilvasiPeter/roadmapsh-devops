First, create a `.env` file for the login route. Then, run the Node.js service:
> Note: See `.env.example` for reference.

```sh
node server.js
```

Build docker image:

```sh
docker build --tag nodeservice --file Dockerfile
```

Run docker container:

```sh
docker run --publish 3000:3000 nodeservice
```

Useful commands:
- `docker ps --all`: List the running container with  to get the container ID. 
- `docker stop <container_id>`: Stop the container.
- `docker container rm <container_id>`: Remove the container.
- `docker image ls`: List container images.
- `docker image <image_id>`: Remove container image.
