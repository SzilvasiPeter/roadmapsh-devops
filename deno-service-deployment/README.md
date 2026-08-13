# Deno Service Deployment

Following the steps will create a simple webserver using (free) custom domain name. It'll be hosted on an Ubuntu server on AWS. In the implementation, we'll use the Deno Javascript runtime and the Ferron webserver. The webserver will configures HTTPS connection bind to the domain name.

# Deploy the EC2 instance

Run the following command:

```sh
terraform apply
```

> Note: It also creates the `inventory.ini` from the `inventory.ini.example` file by replacing the placeholders.

# Create a free domain name

Sing up to the Dynu DDNS: https://www.dynu.com/en-US/ControlPanel/CreateAccount

> Note: Use GitHub or Google authentication for easier account creation.

Get the API key from the https://www.dynu.com/en-US/ControlPanel/APICredentials page.

Get the EC2 public IPv4 address with: `aws ec2 describe-instances --filters "Name=key-name,Values=ec2-key" "Name=instance-state-name,Values=running" --query "Reservations[].Instances[].PublicIpAddress" --output text`

Afterwards, create a free domain name using the POST API call:

```sh
curl -X POST "https://api.dynu.com/v2/dns" \
  -H  "accept: application/json" \
  -H  "API-Key: $DYNU_API_KEY" \
  -H  "Content-Type: application/json" \
  -d '{
    "name": "myjohndoedomainname.ddnsfree.com",
    "group": "",
    "ipv4Address": "'"$EC2_PUBLIC_IPV4"'",
    "ipv6Address": "",
    "ttl": 90,
    "ipv4": true,
    "ipv6": true,
    "ipv4WildcardAlias": true,
    "ipv6WildcardAlias": true,
    "allowZoneTransfer": false,
    "dnssec": false
  }'
```

We'll issue SSL/TLS certificate that binds to this domain name.

# Configure and start the web server

Start Ferron web server with Deno Javascript runtime:

```sh
ansible-playbook -i inventory.ini deno_service.yml
```

It configures HTTPS connection using Let's Encrypt.

See the web server content:

```sh
curl https://myjohndoedomainname.ddnsfree.com/
```

The output should be "Hello, world!" text.

# Clean up

Run the following command:

```sh
terraform destroy
```
