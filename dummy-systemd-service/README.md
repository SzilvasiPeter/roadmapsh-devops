Make the script executable:

```sh
sudo chmod +x ./dummy.sh
```

Create the systemd dummy service:

```sh
sudo nano /etc/systemd/system/dummy.service
```

Add the following content:

```
[Unit]
Description=Dummy Shell Script Service

[Service]
ExecStart=/home/<user_name>/<path_to_script>/dummy.sh
Restart=always

[Install]
WantedBy=multi-user.target
```

Start the dummy service:

```sh
sudo systemctl enable --now dummy.service
```

Stop the dummy service:

```sh
sudo systemctl stop dummy
```

Verify the status with `sudo systemctl status dummy` command.

Disable the dummy service:

```sh
sudo systemctl disable dummy
```

Also, you can remove the original file by executing the `sudo rm /etc/systemd/system/dummy.service` command. At last, clean up the dummy logs:

```sh
sudo rm /var/log/dummy-service.log
```
