PUBLIC_DNS=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicDnsName" \
  --output text)
rsync -avz -e "ssh -i my-ec2-key.pem" ./src/ ec2-user@$PUBLIC_DNS:/usr/share/nginx/html
