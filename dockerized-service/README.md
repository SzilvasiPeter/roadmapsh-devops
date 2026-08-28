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

# GitHub workflow

Create a `terraform.tfvars` file to provide the secrets for the GitHub workflow, then deploy with the command:

```sh
terraform apply
```

Alternatively, use flags to provide the necessary variables:

```sh
terraform apply -var="secret_message=my github secret" -var="username=myuser" -var="password=mypassword"
```

After deployment, run the workflow:

```sh
gh workflow run docker.yml
```

Open the secret page:

```sh
PUBLIC_IP=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicIpAddress" \
  --output text)
xdg-open http://13.53.141.243/secret
```

Login to see the secret message.
