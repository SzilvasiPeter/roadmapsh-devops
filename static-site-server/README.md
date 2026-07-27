# Initialize Linux server

Before creating an EC2 instance in Amazon, be sure to [install AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html#getting-started-install-instructions) then login with the `aws login` command. After that, initialize the remote server:

```sh
source init.sh
```

# Setup Nginx server

## Install Nginx

First connect to the server via SSH:

```sh
PUBLIC_DNS=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicDnsName" \
  --output text)
ssh -i my-ec2-key.pem ec2-user@$PUBLIC_DNS
```

Create the Nginx repository for installation: 

```sh
sudo nano /etc/yum.repos.d/nginx.repo
```

Add the following content:

```
[nginx-stable]
name=nginx stable repo
baseurl=https://nginx.org/packages/amzn/2023/$basearch/
gpgcheck=1
enabled=1
gpgkey=https://nginx.org/keys/nginx_signing.key
module_hotfixes=true
priority=9
```

Save and exit, then install with the following command:

```sh
sudo yum install nginx
```

## Run Nginx

Start the Nginx server as a service:

```sh
sudo systemctl start nginx
```

Ensure that the status is active and running:

```sh
sudo systemctl status nginx
```

## Configure Nginx

Nginx comes with a default website. In the `/etc/nginx/conf.d/default.conf` configuration file, it already defines a server listening on the 80 port. The default static files location is under the `/usr/share/nginx/html` folder. To update the folder content using Rsync command, we need to update ownership of this folder, otherwise we will get permission denied error:

```sh
sudo chown -R ec2-user:ec2-user /usr/share/nginx/html
```

Now, you can exit from the server.

# Deploy static site

You can inspect the default web page before deploying the new content:

```sh
PUBLIC_IP=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicIpAddress" \
  --output text)
curl $PUBLIC_IP
# or open with your default browser
xdg-open http://$PUBLIC_IP
```

Deploy the new content from the `./src` folder this script:

```sh
source deploy.sh
```

Verify that the web page content is changed via `curl` or browser.

# Cleanup resources

Run the following command to delete the resources:

```sh
source delete.sh
```
