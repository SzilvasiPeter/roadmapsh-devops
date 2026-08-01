Before running the Ansible playbook, add the EC2 server hostname in the known hosts:

```sh
PUBLIC_DNS=$(aws ec2 describe-instances \
  --filters "Name=key-name,Values=my-ec2-key" "Name=instance-state-name,Values=running" \
  --query "Reservations[].Instances[].PublicDnsName" \
  --output text)
ssh-keyscan -H $PUBLIC_DNS >> ~/.ssh/known_hosts
```

It enables to connect via SSH without manually accepting the host SSH connection. Otherwise, Ansible couldn't connect to the server:

> [ERROR]: Task failed: Failed to connect to the host via ssh: ssh_askpass: exec(/usr/lib/ssh/ssh-askpass): No such file or directory Host key verification failed.

Next, create the `inventory.ini` file. Copy and paste (`echo $PUBLIC_DNS | xclip`) the server public DNS name to the `ansible_host` parameter. Provide the private key in the `ansible_ssh_private_key_file` parameter. Finally, specify the `ansible_user` parameter (e.g. Amazon Linux -> ec2-user, Ubuntu -> ubuntu, and so on).

Run the playbook to configure the server:

```sh
ansible-playbook -i inventory.ini setup.yml
```

Connect with the newly created SSH key:

```sh
ssh -i ec2-admin-key ec2-user@$PUBLIC_DNS
```
