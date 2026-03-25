mod telemetry_agent;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use telemetry_agent::{LogLevel, parse_log_line, sanitize_log_entry};

fn main() {
    let file: File = File::open("data/sample_logs.txt").expect("Could not open log file");
    let reader = BufReader::new(file);

    let mut log_counts: HashMap<LogLevel, usize> = HashMap::new();

    for line in reader.lines() {
        if let Ok(log_line) = line {
            if let Some(log_entry) = parse_log_line(&log_line) {
                let sanitized_entry = sanitize_log_entry(log_entry);
                println!("{:?}", sanitized_entry);
                *log_counts.entry(sanitized_entry.level).or_insert(0) += 1;
            }
        }
    }

    for (level, count) in log_counts {
        println!("{:?}: {}", level, count);
    }
}
