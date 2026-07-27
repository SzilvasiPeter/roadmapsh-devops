AMI_ID=$(aws ssm get-parameter \
  --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-6.18-x86_64 \
  --query "Parameter.Value" \
  --output text)

aws ec2 create-key-pair \
    --key-name my-ec2-key \
    --query "KeyMaterial" \
    --output text > my-ec2-key.pem
chmod 400 my-ec2-key.pem

MY_IP=$(curl --ipv4 ifconfig.me)
SG_ID=$(aws ec2 create-security-group \
  --group-name ec2-access-sg \
  --description "Allow SSH/HTTP from my IP" \
  --query "GroupId" \
  --output text)
aws ec2 authorize-security-group-ingress \
--group-id $SG_ID \
--protocol tcp \
--port 22 \
--cidr $MY_IP/32
aws ec2 authorize-security-group-ingress \
--group-id $SG_ID \
--protocol tcp \
--port 80 \
--cidr $MY_IP/32

aws ec2 run-instances \
  --image-id $AMI_ID \
  --instance-type t3.micro \
  --key-name my-ec2-key \
  --security-group-ids $SG_ID
