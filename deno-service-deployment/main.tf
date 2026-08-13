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
  }
}

provider "aws" {
  region = "eu-north-1"
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
  description = "Allow SSH from local IP, HTTP and HTTPS from everywhere"
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
  cidr_ipv4         = "0.0.0.0/0"
  from_port         = 80
  to_port           = 80
  ip_protocol       = "tcp"
}

resource "aws_vpc_security_group_ingress_rule" "allow_https" {
  security_group_id = aws_security_group.web_sg.id
  cidr_ipv4         = "0.0.0.0/0"
  from_port         = 443
  to_port           = 443
  ip_protocol       = "tcp"
}

resource "aws_vpc_security_group_egress_rule" "allow_all_outbound" {
  security_group_id = aws_security_group.web_sg.id
  cidr_ipv4         = "0.0.0.0/0"
  ip_protocol       = "-1"
}

resource "aws_instance" "web" {
  ami                    = "ami-0154e061f4582375a" # Ubuntu server 26.04 amd64 hvm ebs-gp3
  instance_type          = "t3.micro"
  key_name               = aws_key_pair.deployer.key_name
  vpc_security_group_ids = [aws_security_group.web_sg.id]
}

# Create the `inventory.ini` file by replacing the public IP and the SSH private key placeholders.
resource "local_file" "inventory" {
  content = replace(
    replace(file("${path.module}/inventory.ini.example"), "<ec2-public-ip>", aws_instance.web.public_ip),
    "<private-key>",
    local_file.private_key.filename,
  )
  filename = "${path.module}/inventory.ini"
}

# Add ec2 instance public IP to known hosts so that ansible can reach the server.
resource "null_resource" "known_hosts" {
  provisioner "local-exec" {
    # Retry ssh-keyscan command, because the OpenSSH service on the instance has not fully started and bound to port 22, after EC2 is created.
    command = "timeout 60s bash -c 'until ssh-keyscan -H ${aws_instance.web.public_ip} >> ~/.ssh/known_hosts 2>/dev/null; do sleep 2; done'"
  }
  depends_on = [aws_instance.web]
}
