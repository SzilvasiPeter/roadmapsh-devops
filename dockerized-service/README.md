# Local deployment

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

# EC2 deployment

Login with AWS CLI, then deploy the EC2 server:

```sh
terraform apply
```

Push docker image to ECR:

```sh
ACCOUNT_ID=$(aws sts get-caller-identity --query Account --output text)
aws ecr get-login-password --region eu-north-1 | docker login --username AWS --password-stdin $ACCOUNT_ID.dkr.ecr.eu-north-1.amazonaws.com
docker build -t nodeservice -f Dockerfile
docker tag localhost/nodeservice $ACCOUNT_ID.dkr.ecr.eu-north-1.amazonaws.com/my-dockerized-service:latest
docker push $ACCOUNT_ID.dkr.ecr.eu-north-1.amazonaws.com/my-dockerized-service:latest
```

Configure the server:

```sh
ansible-playbook docker.yml -i inventory.ini -e "account_id=$ACCOUNT_ID"
```

Clean up resource:

```sh
terraform destroy
```
