mod telemetry_agent;
mod report;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use telemetry_agent::{LogEntry, LogLevel, parse_log_line, sanitize_log_entry};
use report::{LogReport, Report};

fn main() {
    let file: File = File::open("data/sample_logs.txt").expect("Could not open log file");
    let reader = BufReader::new(file);
    let mut log_counts: HashMap<LogLevel, usize> = HashMap::new();
    let mut sanitized_entries: Vec<LogEntry> = Vec::new();

    for line in reader.lines() {
        if let Ok(log_line) = line {
            if let Some(log_entry) = parse_log_line(&log_line) {
                let sanitized_entry = sanitize_log_entry(log_entry);
                *log_counts.entry(sanitized_entry.level).or_insert(0) += 1;
                sanitized_entries.push(sanitized_entry);
            }
        }
    }

    // Generate and print report
    let report = LogReport {
        log_entries: sanitized_entries,
        log_level_counts: log_counts,
    };
    println!("\nDetailed Log Report:\n\n{}", report.generate_report());
}
