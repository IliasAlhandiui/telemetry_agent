# Telemetry Agent

A high-performance Rust-based log processing and reporting tool, designed to efficiently parse, sanitize, and summarize large-scale log data. Includes a Python script for generating realistic sample logs.

## Features
- **Log Parsing:** Reads logs in the format `[TIMESTAMP] LEVEL IP - MESSAGE`.
- **Sanitization:** Masks sensitive data such as email addresses and credit card numbers in log messages.
- **Log Level Analysis:** Counts occurrences of each log level (INFO, DEBUG, WARN, ERROR, CRITICAL).
- **Reporting:**
  - Generates a detailed text report with sample log entries and log level summary.
  - Exports a JSON report for further analysis or integration.
- **Sample Data Generation:** Python script to create large, realistic log files with optional PII for testing.
