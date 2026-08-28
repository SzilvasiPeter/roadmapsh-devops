terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 6.0"
    }
    null = {
      source  = "hashicorp/null"
      version = "~> 3.0"
    }
    github = {
      source  = "integrations/github"
      version = "~> 6.0"
    }
  }
}

provider "aws" {
  region = "eu-north-1"
}

provider "github" {
  owner = "SzilvasiPeter"
}

data "github_repository" "repo" {
  full_name = "SzilvasiPeter/roadmapsh-devops"
}

data "github_user" "owner" {
  username = "SzilvasiPeter"
}

data "aws_ami" "amazon_linux_2023" {
  most_recent = true
  owners      = ["amazon"]

  filter {
    name   = "name"
    values = ["al2023-ami-*-x86_64"]
  }
}

data "http" "my_ip" {
  url = "https://checkip.amazonaws.com"
}

locals {
  my_cidr = "${trimspace(data.http.my_ip.response_body)}/32"
}

resource "tls_private_key" "ssh" {
  algorithm = "ED25519"
}

resource "aws_key_pair" "deployer" {
  key_name   = "ec2-key"
  public_key = tls_private_key.ssh.public_key_openssh
}

resource "local_file" "private_key" {
  content         = tls_private_key.ssh.private_key_pem
  filename        = "./ec2-key.pem"
  file_permission = "0600"
}

resource "aws_default_vpc" "default" {}

resource "aws_security_group" "web_sg" {
  name        = "allow-local"
  description = "Allow SSH, HTTP  from local public IP"
  vpc_id      = aws_default_vpc.default.id
}

resource "aws_vpc_security_group_ingress_rule" "allow_ssh" {
  security_group_id = aws_security_group.web_sg.id
  cidr_ipv4         = local.my_cidr
  from_port         = 22
  to_port           = 22
  ip_protocol       = "tcp"
}

resource "aws_vpc_security_group_ingress_rule" "allow_http" {
  security_group_id = aws_security_group.web_sg.id
  cidr_ipv4         = local.my_cidr
  from_port         = 80
  to_port           = 80
  ip_protocol       = "tcp"
}

resource "aws_vpc_security_group_egress_rule" "allow_all_outbound" {
  security_group_id = aws_security_group.web_sg.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1"
}

resource "aws_iam_role" "ec2_ecr_role" {
  name = "ec2-ecr-read-only-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ec2.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_instance_profile" "ec2_profile" {
  name = "ec2-ecr-read-only-and-ssm-management"
  role = aws_iam_role.ec2_ecr_role.name
}

resource "aws_iam_role_policy_attachment" "ecr_read_only" {
  role       = aws_iam_role.ec2_ecr_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryReadOnly"
}

resource "aws_iam_role_policy_attachment" "ssm_policy" {
  role       = aws_iam_role.ec2_ecr_role.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore"
}

resource "aws_instance" "docker_ec2" {
  ami                    = data.aws_ami.amazon_linux_2023.id
  instance_type          = "t3.micro"
  key_name               = aws_key_pair.deployer.key_name
  vpc_security_group_ids = [aws_security_group.web_sg.id]
  iam_instance_profile   = aws_iam_instance_profile.ec2_profile.name
  user_data              = <<-EOF
                #!/bin/bash
                dnf update -y
                dnf install -y docker
                systemctl start docker
                systemctl enable docker
                # Add ec2-user to docker group to run docker commands without sudo
                usermod -aG docker ec2-user
                EOF
}

resource "aws_ecr_repository" "app" {
  name                 = "my-dockerized-service"
  image_tag_mutability = "MUTABLE"
  force_delete         = true

  image_scanning_configuration {
    scan_on_push = true
  }
}

# --- Ansible preconfiguration ---
# Create the `inventory.ini` file by replacing the public IP and the SSH private key placeholders.
resource "local_file" "inventory" {
  content = replace(
    replace(file("${path.module}/inventory.ini.example"), "<ec2-public-ip>", aws_instance.docker_ec2.public_ip),
    "<private-key>",
    local_file.private_key.filename,
  )
  filename = "${path.module}/inventory.ini"
}

# Add ec2 instance public IP to known hosts so that ansible can reach the server.
resource "null_resource" "known_hosts" {
  provisioner "local-exec" {
    command = "ssh-keyscan -H ${aws_instance.docker_ec2.public_ip} >> ~/.ssh/known_hosts"
  }
  depends_on = [aws_instance.docker_ec2]
}

# --- OIDC configuration for running ECR and SSM command on GitHub workflow. ---
resource "aws_iam_openid_connect_provider" "github" {
  url            = "https://token.actions.githubusercontent.com"
  client_id_list = ["sts.amazonaws.com"]
}

resource "aws_iam_role" "github_actions" {
  name = "github-action-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect = "Allow"
        Principal = {
          Federated = aws_iam_openid_connect_provider.github.arn
        }
        Action = "sts:AssumeRoleWithWebIdentity"
        Condition = {
          StringEquals = {
            "token.actions.githubusercontent.com:aud" = "sts.amazonaws.com"
            # The owner and repo ID is mandatory for repos created after 15 July 2026, see: https://github.blog/changelog/2026-04-23-immutable-subject-claims-for-github-actions-oidc-tokens/
            "token.actions.githubusercontent.com:sub" = "repo:SzilvasiPeter@${data.github_user.owner.id}/roadmapsh-devops@${data.github_repository.repo.repo_id}:ref:refs/heads/main"
          }
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "ecr_power_user" {
  role       = aws_iam_role.github_actions.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonEC2ContainerRegistryPowerUser"
}

resource "aws_iam_role_policy_attachment" "ssm_full_access" {
  role       = aws_iam_role.github_actions.name
  policy_arn = "arn:aws:iam::aws:policy/AmazonSSMFullAccess"
}

resource "github_actions_secret" "aws_role_arn" {
  repository  = data.github_repository.repo.name
  secret_name = "AWS_ROLE_ARN"
  value       = aws_iam_role.github_actions.arn
}

resource "github_actions_secret" "ec2_instance_id" {
  repository  = data.github_repository.repo.name
  secret_name = "EC2_INSTANCE_ID"
  value       = aws_instance.docker_ec2.id
}

variable "secret_message" {
  type      = string
  sensitive = true
}

variable "username" {
  type      = string
  sensitive = true
}

variable "password" {
  type      = string
  sensitive = true
}

resource "github_actions_secret" "secret_message" {
  repository  = data.github_repository.repo.name
  secret_name = "SECRET_MESSAGE"
  value       = var.secret_message
}

resource "github_actions_secret" "username" {
  repository  = data.github_repository.repo.name
  secret_name = "USERNAME"
  value       = var.username
}

resource "github_actions_secret" "password" {
  repository  = data.github_repository.repo.name
  secret_name = "PASSWORD"
  value       = var.password
}
