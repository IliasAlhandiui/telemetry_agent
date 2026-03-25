use crate::telemetry_agent::{LogEntry, LogLevel};
use std::collections::HashMap;
pub trait Report {
    fn generate_report(&self) -> String;
}

pub struct LogReport {
    pub log_entries: Vec<LogEntry>,
    pub log_level_counts: HashMap<LogLevel, usize>,
}

impl Report for LogReport {
    fn generate_report(&self) -> String {
        let mut report = String::new();
        //print 10 log entries as sample
        report.push_str("Sample Log Entries:\n");
        for entry in self.log_entries.iter().take(10) {
            report.push_str(&format!(
                "{} [{}] {} - {}\n",
                entry.timestamp, entry.level as u8, entry.ip, entry.message
            ));
        }
        report.push_str("\nLog Level Summary:\n");
        for (level, count) in &self.log_level_counts {
            report.push_str(&format!("{:?}: {}\n", level, count));
        }
        report
    }
}
