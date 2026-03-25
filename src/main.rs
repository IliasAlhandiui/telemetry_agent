mod report;
mod telemetry_agent;
use report::{JSONReport, LogReport, Report};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use telemetry_agent::{LogEntry, LogLevel, parse_log_line, sanitize_log_entry};

fn main() {
    let file: File = File::open("data/sample_logs.txt").expect("Could not open log file");
    let reader = BufReader::new(file);
    let mut log_counts: HashMap<LogLevel, usize> = HashMap::new();
    let mut sanitized_entries: Vec<LogEntry> = Vec::new();

    for line in reader.lines() {
        if let Ok(log_line) = line {
            if let Some(log_entry) = parse_log_line(&log_line) {
                let sanitized_entry = sanitize_log_entry(log_entry, false);
                *log_counts.entry(sanitized_entry.level).or_insert(0) += 1;
                sanitized_entries.push(sanitized_entry);
            }
        }
    }

    // Generate and print report
    let report = LogReport::new(&sanitized_entries, &log_counts);
    println!("\nDetailed Log Report:\n\n{}", report.generate_report());

    // Generate and export JSON report
    let json_report = JSONReport::new(&sanitized_entries, &log_counts);
    let json_output = json_report.generate_report();
    std::fs::write("data/log_report.json", json_output).expect("Could not write JSON report");
    println!("JSON report generated: data/log_report.json");
}
