Deploy the EC2 instance:

```sh
terraform apply
```

> Note: It also creates the `inventory.ini` from the `inventory.ini.example` file by replacing the placeholders.

Install Nginx server:

```sh
ansible-playbook -i inventory.ini nginx.yml
```

See the default webpage content:

```sh
PUBLIC_DNS=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicDnsName" \
  --output text)
curl $PUBLIC_DNS
```

Clean up resources:

```sh
terraform destroy
```
