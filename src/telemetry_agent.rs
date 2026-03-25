#[derive(Debug)]
pub enum LogLevel {
    INFO,
    DEBUG,
    WARN,
    ERROR,
    CRITICAL,
}

#[derive(Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub ip: String,
    pub message: String,
}

pub fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.splitn(4, ' ').collect();
    if parts.len() < 4 {
        return None;
    }

    let timestamp = parts[0].trim_matches(|c| c == '[' || c == ']');
    let level_str = parts[1];
    let ip = parts[2];
    let message = parts[3].trim_start_matches("- ").trim();

    let level = match level_str {
        "INFO" => LogLevel::INFO,
        "DEBUG" => LogLevel::DEBUG,
        "WARN" => LogLevel::WARN,
        "ERROR" => LogLevel::ERROR,
        "CRITICAL" => LogLevel::CRITICAL,
        _ => return None,
    };

    Some(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        ip: ip.to_string(),
        message: message.to_string(),
    })
}

pub fn sanitize_log_entry(log_entry: LogEntry) -> LogEntry {
    // Simple sanitization: email addresses and potentiall credit card numbers with ***
    let email_regex =
        regex::Regex::new(r"([a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,})").unwrap();
    let cc_regex = regex::Regex::new(r"\b(?:\d[ -]*?){13,16}\b").unwrap();

    let sanitized_message = email_regex
        .replace_all(&log_entry.message, "***")
        .to_string();
    let sanitized_message = cc_regex.replace_all(&sanitized_message, "***").to_string();

    LogEntry {
        timestamp: log_entry.timestamp,
        level: log_entry.level,
        ip: log_entry.ip,
        message: sanitized_message,
    }
}
