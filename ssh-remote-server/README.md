# Create AWS account

Requirements:
- Email address
- Credit card

Select the free tier plan that gives $100 credit to spend.

# Create an EC2 instance 

## AWS Console Home

Launch an instance
---

1. Search for the "ec2" compute service, and select **EC2 Virtual Servers in the Cloud** search result
2. Click on the **Launch Instance** button
3. Scroll down to **Network settings** and change the **Allow SSH traffic from** value from "Anywhere" to "My IP"
4. Keep all the other default values (AMI: Amazon Linux 2023 kernel-6.18, Instance type: t3.micro, Storage: 8 GiB gp3 as of 2026 July)
5. In the **Summary** panel, click on the **Launch Instance** button.

Create a key pair
---

1. Add a key pair name such as `my-key-pair-name-1`
2. Keep the default values (Key pair type: RSA, Private key file format: .pem)
3. Save the `.pem` file

Connect to SSH
---

1. Go to your instance (something like `i-03abc000fb7b13fe5`)
2. Click on the **Connect** button
3. Select the **In SSH client** tab
4. Follow the **How to connect** steps

The final output should look like this:

```
$ ssh -i "my-key-pair-1.pem" ec2-user@ec2-13-63-126-31.eu-north-1.compute.amazonaws.com
** WARNING: connection is not using a post-quantum key exchange algorithm.
** This session may be vulnerable to "store now, decrypt later" attacks.
** The server may need to be upgraded. See https://openssh.com/pq.html
   ,     #_
   ~\_  ####_        Amazon Linux 2023
  ~~  \_#####\
  ~~     \###|
  ~~       \#/ ___   https://aws.amazon.com/linux/amazon-linux-2023
   ~~       V~' '->
    ~~~         /
      ~~._.   _/
         _/ _/
       _/m/'
```

(Optional) Delete the instance
---

1. Go back to the instance
2. Under the **Instance state** drop down menu, select the **Terminate (delete) instance** item

## AWS CLI

Install AWS CLI on Linux
---

Executing the following command:

```sh
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
./aws/install --install-dir ~/.local/aws-cli --bin-dir ~/.local/bin/
```

Use the `--install-dir` and `--bin-dir` to install without sudo. Make sure that `~/.local/bin` is imported in the `PATH` environment variable.

References:
- Install guide: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
- AWS CLI docs: https://docs.aws.amazon.com/cli/latest/reference/

Login to AWS
---

First, authenticate the AWS CLI by login via web browser:

```sh
aws login
```

Simplest way is to login as root user using email address.

Get the image ID
---

Get the latest image ID for the Amazon Linux 2023 kernel 6.18 x86_64 architecture:

```sh
AMI_ID=$(aws ssm get-parameter \
  --name /aws/service/ami-amazon-linux-latest/al2023-ami-kernel-6.18-x86_64 \
  --query "Parameter.Value" \
  --output text)
```

> Note: You can also navigate to the **EC2** > **AMI Catalog** to verify the Amazon Machine Image (AMI).

Generate key pair
---

Generate a key pair to connect to the server via SSH client:

```sh
aws ec2 create-key-pair \
    --key-name my-ec2-key \
    --query "KeyMaterial" \
    --output text > my-ec2-key.pem
```

Set read permission, since you cannot connect to your instance using unprotected key:

```sh
chmod 400 my-ec2-key.pem
```

Create Security Group & Allow SSH for Your IP
---

Detect your current public IP address:

```sh
MY_IP=$(curl --ipv4 ifconfig.me)
```

Create a security group:

```sh
SG_ID=$(aws ec2 create-security-group \
  --group-name ssh-access-sg \
  --description "Allow SSH from my IP" \
  --query "GroupId" \
  --output text)
```

Add inbound SSH rule:

```sh
aws ec2 authorize-security-group-ingress \
  --group-id $SG_ID \
  --protocol tcp \
  --port 22 \
  --cidr $MY_IP/32
```

Launch the EC2 Instance
---

Run an EC2 instance using the free tier eligible `t3.micro` instance type:

```sh
aws ec2 run-instances \
  --image-id $AMI_ID \
  --instance-type t3.micro \
  --key-name my-ec2-key \
  --security-group-ids $SG_ID
```

Connect via SSH
---

Get the Public DNS:

```sh
PUBLIC_DNS=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicDnsName" \
  --output text)
```

Connect using the saved key:

```sh
ssh -i my-ec2-key.pem ec2-user@$PUBLIC_DNS
```

> Note: The default user is `ec2-user` for Amazon Linux 2023 machine.

(Optional) Clean up the resources
---

```sh
# 1. Terminate the EC2 instance
INSTANCE_ID=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running,pending,stopped" \
  --query "Reservations[].Instances[].InstanceId" \
  --output text)

aws ec2 terminate-instances --instance-ids $INSTANCE_ID

# 2. Wait for the instance to fully terminate (Security Group cannot be deleted while in use)
aws ec2 wait instance-terminated --instance-ids $INSTANCE_ID

# 3. Delete the Security Group
aws ec2 delete-security-group --group-name ssh-access-sg

# 4. Delete the SSH Key Pair from AWS & local file
aws ec2 delete-key-pair --key-name my-ec2-key
rm --force my-ec2-key.pem
```
