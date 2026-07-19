# Build

```
cargo build -p log-archive --release
```

# Run

User space log files:

```
./target/release/log-archive ~/.local/share/zed/logs/
```

Kernel space (root) log files needs sudo privilege:

```
sudo ./target/release/log-archive /var/log
```
