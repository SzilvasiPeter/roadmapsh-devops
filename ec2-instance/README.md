# EC2 instance with custom domain and free SSL/TLS certificate

Deploy the EC2 instance with Terraform:

```sh
terraform apply
```

Use this command to connect to the server via SSH:

```sh
PUBLIC_DNS=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicDnsName" \
  --output text)
ssh -i "ec2-key.pem" ubuntu@$PUBLIC_DNS
```

Create a free domain name service:

1. Sign up the https://freedns.afraid.org/signup/ webpage
2. Login and go to the https://freedns.afraid.org/subdomain/ page
3. Add a new subdomain
4. Fill the form: in the "Destination" input field paste in the EC2 public IPv4 address.

> Note: To get the public IP address, run the `aws ec2 describe-instances --filters "Name=key-name,Values=ec2-key" "Name=instance-state-name,Values=running" --query "Reservations[].Instances[].PublicIpAddress" --output text` command.

Install and start Nginx:

```sh
sudo apt update
sudo apt install -y nginx
sudo systemctl start nginx
```

Install `certbot` to configure HTTPS using Let's Encrypt:

```sh
sudo apt install certbot python3-certbot-nginx -y
```

By default the 80 port is blocked by firewall, enable it to allow Let's Encrypt connection.

```sh
sudo ufw allow 80/tcp
```

Register the newly created domain:

```sh
sudo certbot --nginx -d myspecialjohndoedomain.mooo.com
```

The output should look like this:

```
Saving debug log to /var/log/letsencrypt/letsencrypt.log
Requesting a certificate for myspecialjohndoedomain.mooo.com

Successfully received certificate.
Certificate is saved at: /etc/letsencrypt/live/myspecialjohndoedomain.mooo.com/fullchain.pem
Key is saved at:         /etc/letsencrypt/live/myspecialjohndoedomain.mooo.com/privkey.pem
This certificate expires on 2026-10-28.
These files will be updated when the certificate renews.
Certbot has set up a scheduled task to automatically renew this certificate in the background.

Deploying certificate
Successfully deployed certificate for myspecialjohndoedomain.mooo.com to /etc/nginx/sites-enabled/default
Congratulations! You have successfully enabled HTTPS on https://myspecialjohndoedomain.mooo.com

- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
If you like Certbot, please consider supporting our work by:
 * Donating to ISRG / Let's Encrypt:   https://letsencrypt.org/donate
 * Donating to EFF:                    https://eff.org/donate-le
- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
```

> Note: If the server public IP is changed, you need to update your subdomain ("Destination" value) in FreeDNS. You may need to refresh the DNS records with the `sudo resolvectl flush-caches` command to reflect the changes.

Grant ownership for the Nginx default public web root location to copy file without `sudo` permission:

```sh
sudo chown -R ubuntu:ubuntu /var/www/html/
```

Copy the simple HTML file from your machine to the server:

```sh
scp -i ec2-key.pem index.html ubuntu@$PUBLIC_DNS:/var/www/html/
```

Go to the https://myspecialjohndoedomain.mooo.com page in the browser to verify the content change.

Clean up the resources:

```sh
terraform destroy
```

Useful commands:

```sh
# Get the latest Ubuntu Server 26.04 amd64 image
aws ssm get-parameter --name /aws/service/canonical/ubuntu/server/26.04/stable/current/amd64/hvm/ebs-gp3/ami-id --query "Parameter.Value" --output text

# Format the .tf file
terraform fmt main.tf

# Validate the .tf file
terraform validate
```
