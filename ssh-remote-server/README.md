# Create AWS account

Requirements:
- Email address
- Credit card

Select the free tier plan that gives $100 credit to spend.

# Create an EC2 instance (AWS Console Home)

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
