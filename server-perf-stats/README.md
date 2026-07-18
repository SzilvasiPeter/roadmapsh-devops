# Build

```
cargo build --release
```

# Run

```
./server-stats.sh
```

# Output

```
--- CPU ---
Total usage: 19.29%

--- Memory ---
Total: 15.53 GiB
Free: 7.51 GiB
Used: 8.02 GiB
Percentage: 51.63%

--- Disk ---
Total: 457.39 GiB
Free: 271.11 GiB
Used: 186.28 GiB
Percentage: 40.73%

--- Top 5 Processes by CPU Usage ---
1. PID: 47105, Name: server-perf-sta, CPU: 9.78%
2. PID: 407, Name: kworker/4:2-events, CPU: 3.26%
3. PID: 15346, Name: Worker-7, CPU: 0.00%
4. PID: 14617, Name: module-rt, CPU: 0.00%
5. PID: 16465, Name: Worker, CPU: 0.00%

--- Top 5 Processes by Memory Usage ---
1. PID: 16294, Name: opencode, Memory: 75451.00 MiB
2. PID: 14810, Name: librewolf, Memory: 19522.00 MiB
3. PID: 15338, Name: zed-editor, Memory: 18781.00 MiB
4. PID: 46337, Name: node, Memory: 18075.00 MiB
5. PID: 15529, Name: rust-analyzer, Memory: 3101.00 MiB

--- OS ---
Name: Arch Linux
Version: 7.1.3-arch1-3
```
