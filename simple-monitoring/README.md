# Install Netdata

On Linux system, run the following command to install Netdata:

```sh
setup.sh
```

# Customize dashboard

Go to http://localhost:19999 then click on the "Skip and use the dashboard anonymously." link under the **Sign-in** button.

Go to the **Dashboard** tab and add the "Server Deep Dive" template that displays comprehensive single-server metrics.

# Test Dashboard

Utilize the CPU by calculating prime numbers with this command:

```sh
source test_dashboard.sh
```

You'll see a spike in the `Total CPU utilization` graph.
