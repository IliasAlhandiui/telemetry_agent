use crate::telemetry_agent::{LogEntry, LogLevel};
use std::collections::HashMap;
pub trait Report {
    fn generate_report(&self) -> String;
}

pub struct LogReport<'a> {
    log_entries: &'a [LogEntry],
    log_level_counts: &'a HashMap<LogLevel, usize>,
}

impl Report for LogReport<'_> {
    fn generate_report(&self) -> String {
        let mut report = String::new();

        //print 10 log entries as sample
        report.push_str("Sample Log Entries:\n");
        for entry in self.log_entries.iter().take(10) {
            report.push_str(&format!(
                "{} [{:?}] {} - {}\n",
                entry.timestamp, entry.level, entry.ip, entry.message
            ));
        }
        if self.log_entries.len() > 10 {
            println!("... ({} more entries)\n", self.log_entries.len() - 10);
        }

        report.push_str("\nLog Level Summary:\n");
        for (level, count) in self.log_level_counts {
            report.push_str(&format!("{:?}: {}\n", level, count));
        }

        report
    }
}

impl<'a> LogReport<'a> {
    pub fn new(
        log_entries: &'a [LogEntry],
        log_level_counts: &'a HashMap<LogLevel, usize>,
    ) -> Self {
        // only save first 10 log entries for the report to avoid overwhelming output
        if log_entries.len() > 10 {
            println!(
                "LogReport: Received {} log entries, only showing first 10 in report",
                log_entries.len()
            );
        }

        let log_entries = if log_entries.len() > 10 {
            &log_entries[..10]
        } else {
            log_entries
        };

        LogReport {
            log_entries,
            log_level_counts,
        }
    }
}

pub struct JSONReport<'a> {
    log_entries: &'a [LogEntry],
    log_level_counts: &'a HashMap<LogLevel, usize>,
}

impl Report for JSONReport<'_> {
    fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("{\n  \"log_entries\": [\n");

        for entry in self.log_entries.iter() {
            report.push_str(&format!(
                "    {{\"timestamp\": \"{}\", \"level\": \"{:?}\", \"ip\": \"{}\", \"message\": \"{}\"}},\n",
                entry.timestamp, entry.level, entry.ip, entry.message
            ));
        }

        report.push_str("  ],\n  \"log_level_counts\": {\n");
        for (level, count) in self.log_level_counts {
            report.push_str(&format!("    \"{:?}\": {},\n", level, count));
        }
        report.push_str("  }\n}");

        report
    }
}

impl<'a> JSONReport<'a> {
    pub fn new(
        log_entries: &'a [LogEntry],
        log_level_counts: &'a HashMap<LogLevel, usize>,
    ) -> Self {
        JSONReport {
            log_entries,
            log_level_counts,
        }
    }
}
