use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

fn main() {
    let refresh = RefreshKind::nothing().with_cpu(CpuRefreshKind::everything());
    let mut sys = System::new_with_specifics(refresh);

    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_all();
    let cpu_usage: f32 = sys.cpus().iter().map(sysinfo::Cpu::cpu_usage).sum();
    println!("Total CPU usage: {cpu_usage:.2}%");

    let refresh = MemoryRefreshKind::nothing().with_ram();
    sys.refresh_memory_specifics(refresh);
    let memory_total = sys.total_memory();
    let memory_free = sys.free_memory();
    let memory_used = memory_total - memory_free;
    let total_gib = f64::from(u32::try_from(memory_total / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let free_gib = f64::from(u32::try_from(memory_free / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let used_gib = f64::from(u32::try_from(memory_used / 1024 / 1024).unwrap_or(0)) / 1024.0;
    let percentage = (used_gib / total_gib) * 100.0;
    println!("Total memory: {total_gib:.2} GiB");
    println!("Free memory: {free_gib:.2} GiB");
    println!("Used memory: {used_gib:.2} GiB");
    println!("Percentage memory: {percentage:.2}%");
}
