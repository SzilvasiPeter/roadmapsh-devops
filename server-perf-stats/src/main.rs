use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

fn main() {
    let refresh = RefreshKind::nothing().with_cpu(CpuRefreshKind::everything());
    let mut sys = System::new_with_specifics(refresh);
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    let cpu_usage: f32 = sys.cpus().iter().map(sysinfo::Cpu::cpu_usage).sum();
    println!("--- CPU ---");
    println!("Total CPU usage: {cpu_usage:.2}%");

    let refresh = MemoryRefreshKind::nothing().with_ram();
    sys.refresh_memory_specifics(refresh);
    let memory_total = sys.total_memory();
    let memory_free = sys.free_memory();
    let total_gib = f64::from(u32::try_from(memory_total / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let free_gib = f64::from(u32::try_from(memory_free / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let used_gib = total_gib - free_gib;
    let percentage = (used_gib / total_gib) * 100.0;
    println!("\n--- Memory ---");
    println!("Total: {total_gib:.2} GiB");
    println!("Free: {free_gib:.2} GiB");
    println!("Used: {used_gib:.2} GiB");
    println!("Percentage: {percentage:.2}%");

    let disks = Disks::new_with_refreshed_list();
    let disk_total: u64 = disks.iter().map(sysinfo::Disk::total_space).sum();
    let disk_free: u64 = disks.iter().map(sysinfo::Disk::available_space).sum();
    let disk_total_gib = f64::from(u32::try_from(disk_total / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let disk_free_gib = f64::from(u32::try_from(disk_free / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let disk_used_gib = disk_total_gib - disk_free_gib;
    let disk_percentage = (disk_used_gib / disk_total_gib) * 100.0;
    println!("\n--- Disk ---");
    println!("Total: {disk_total_gib:.2} GiB");
    println!("Free: {disk_free_gib:.2} GiB");
    println!("Used: {disk_used_gib:.2} GiB");
    println!("Percentage: {disk_percentage:.2}%");

    let mut sys = System::new_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    let refresh = ProcessRefreshKind::nothing().with_cpu().with_memory();
    sys.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, refresh);
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());
    println!("\n--- Top 5 Processes by CPU Usage ---");
    for (i, (pid, proc)) in processes.iter().take(5).enumerate() {
        let i = i + 1;
        let name = proc.name().to_string_lossy();
        let usage = proc.cpu_usage();
        println!("{i}. PID: {pid}, Name: {name}, CPU: {usage:.2}%");
    }

    let mut sys = System::new_all();
    let refresh = ProcessRefreshKind::nothing().with_memory();
    sys.refresh_processes_specifics(sysinfo::ProcessesToUpdate::All, true, refresh);
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| {
        b.1.virtual_memory()
            .partial_cmp(&a.1.virtual_memory())
            .unwrap()
    });

    println!("\n--- Top 5 Processes by Memory Usage ---");
    let mut seen: Vec<String> = Vec::with_capacity(5);
    let mut count = 0;
    for (pid, proc) in &processes {
        let name = proc.exe().and_then(|p| p.file_name()).map_or_else(
            || "Unknown".to_string(),
            |f| f.to_string_lossy().into_owned(),
        );

        if seen.contains(&name) {
            continue;
        }
        seen.push(name.clone());
        count += 1;

        let memory = f64::from(u32::try_from(proc.virtual_memory() / 1024 / 1024).unwrap_or(0));
        println!("{count}. PID: {pid}, Name: {name}, Memory: {memory:.2} MiB");

        if count == 5 {
            break;
        }
    }
}
