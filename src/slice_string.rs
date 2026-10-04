pub fn extract_severity(log: &str) -> &str {
    let start = log.find('[');
    if start.is_none() {
        return log;
    }
    let start = start.unwrap() + 1;
    let end = log[start..].find(']');
    if end.is_none() {
        return log;
    }
    let end = start + end.unwrap();
    &log[start..end]
}