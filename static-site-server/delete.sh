INSTANCE_ID=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running,pending,stopped" \
  --query "Reservations[].Instances[].InstanceId" \
  --output text)
aws ec2 terminate-instances --instance-ids $INSTANCE_ID
aws ec2 wait instance-terminated --instance-ids $INSTANCE_ID

aws ec2 delete-security-group --group-name ec2-access-sg
aws ec2 delete-key-pair --key-name my-ec2-key
rm --force my-ec2-key.pem
