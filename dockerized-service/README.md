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

Create provider to authenticate between GitHub workflow and AWS:

```sh
aws iam create-open-id-connect-provider \
  --url "https://token.actions.githubusercontent.com" \
  --client-id-list "sts.amazonaws.com"
```

Create role that trust the GitHub provider to obtain OIDC connection:

```sh
AWS_ACCOUNT_ID=$(aws sts get-caller-identity --query "Account" --output text)
OWNER_ID=$(gh api user --jq '.id')
REPO_ID=$(gh api repos/SzilvasiPeter/roadmapsh-devops --jq '.id')

aws iam create-role \
  --role-name github-action-role \
  --assume-role-policy-document "$(cat <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Principal": {
        "Federated": "arn:aws:iam::${AWS_ACCOUNT_ID}:oidc-provider/token.actions.githubusercontent.com"
      },
      "Action": "sts:AssumeRoleWithWebIdentity",
      "Condition": {
        "StringEquals": {
          "token.actions.githubusercontent.com:aud": "sts.amazonaws.com",
          "token.actions.githubusercontent.com:sub": "repo:SzilvasiPeter@${OWNER_ID}/roadmapsh-devops@${REPO_ID}:ref:refs/heads/main"
        }
      }
    }
  ]
}
EOF
)"
```

Assign policies for ECR (pushing docker image) and SSM (pulling docker image) related commands:

```sh
aws iam attach-role-policy \
  --role-name github-action-role \
  --policy-arn arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryPowerUser

aws iam attach-role-policy \
  --role-name github-action-role \
  --policy-arn arn:aws:iam::aws:policy/AmazonSSMFullAccess
```

Create or update the secrets for the workflow:

```sh
AWS_ROLE_ARN=$(aws iam get-role --role-name github-action-role --query "Role.Arn" --output text)
EC2_INSTANCE_ID=$(aws ec2 describe-instances --filters "Name=key-name,Values=ec2-key" "Name=instance-state-name,Values=running" --query "Reservations[0].Instances[0].InstanceId" --output text)

gh secret set AWS_ROLE_ARN --body "$AWS_ROLE_ARN" --repo SzilvasiPeter/roadmapsh-devops
gh secret set EC2_INSTANCE_ID --body "$EC2_INSTANCE_ID" --repo SzilvasiPeter/roadmapsh-devops
gh secret set SECRET_MESSAGE --body "my github secret" --repo SzilvasiPeter/roadmapsh-devops
gh secret set USERNAME --body "myuser" --repo SzilvasiPeter/roadmapsh-devops
gh secret set PASSWORD --body "mypassword" --repo SzilvasiPeter/roadmapsh-devops
```

Now, run the workflow by changing one of the source files (e.g. `login.html`) or trigger it manually.
