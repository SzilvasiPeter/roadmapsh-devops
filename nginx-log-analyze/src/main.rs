use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq)]
pub struct LogEntry {
    pub ip: String,
    pub datetime: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub size: u64,
    pub referrer: String,
    pub user_agent: String,
}

impl LogEntry {
    #[must_use]
    pub fn parse(line: &str) -> Option<Self> {
        let mut parts = line.split('"');

        let prefix = parts.next()?;
        let ip = prefix.split_whitespace().next()?.to_string();
        let datetime = prefix.split('[').nth(1)?.split(']').next()?.to_string();

        let req_str = parts.next()?;
        let mut req_parts = req_str.split_whitespace();
        let method = req_parts.next()?.to_string();
        let path = req_parts.next()?.to_string();

        let mut status_bytes = parts.next()?.split_whitespace();
        let status = status_bytes.next()?.parse().ok()?;
        let size = status_bytes.next()?.parse().unwrap_or(0);

        let referrer = parts.next()?.to_string();
        parts.next();
        let user_agent = parts.next()?.to_string();

        Some(Self {
            ip,
            datetime,
            method,
            path,
            status,
            size,
            referrer,
            user_agent,
        })
    }
}

fn main() {
    let log = fs::read_to_string("./nginx-log-analyze/nginx-access.log").unwrap();
    let (valid, failed): (Vec<_>, Vec<_>) = log
        .lines()
        .enumerate()
        .partition(|(_, line)| LogEntry::parse(line).is_some());

    println!("Parsed successfully: {}", valid.len());
    println!("Failed to parse: {}", failed.len());

    let entries: Vec<LogEntry> = valid
        .iter()
        .filter_map(|(_, line)| LogEntry::parse(line))
        .collect();

    top_n(&entries, |e| e.ip.clone(), 5, "IP addresses");
    top_n(&entries, |e| e.path.clone(), 5, "requested paths");
    top_n(&entries, |e| e.status, 5, "response status codes");
    top_n(&entries, |e| e.user_agent.clone(), 5, "user agents");
}

fn top_n<K: Hash + Eq + Display>(
    entries: &[LogEntry],
    extract: impl Fn(&LogEntry) -> K,
    n: usize,
    label: &str,
) {
    let mut counts: HashMap<K, usize> = HashMap::new();
    for entry in entries {
        *counts.entry(extract(entry)).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = counts.into_iter().collect();
    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));

    println!("\nTop {n} {label}:");
    for (key, count) in sorted.iter().take(n) {
        println!("{key} - {count} requests");
    }
}
