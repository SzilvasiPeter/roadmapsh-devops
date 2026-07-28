Build the docker image:

```sh
docker build -t hello-captain .
```

> Note: If you have `podman` you can alias it as `docker` command, since they are compatible.

List the created docker image:

```sh
docker image ls
```

You'll also see the downloaded alpine image. This is useful since the docker layers are cached and reused, helping the build time and increase isolation.

Run the docker image:

```sh
docker run hello-captain
```

The output should look like this: "Hello, Captain!"

Delete the container (running image instance) and the image:

```sh
docker image rm --force localhost/hello-captain
```
