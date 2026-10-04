
mod calculator;
mod slice_string;
use crate::slice_string::extract_severity;
fn main() {
    calculator::calc();
    let log = String::from("[WARN] Disk space reaching 90%");
    let severity = extract_severity(&log);
    println!("Severity: {severity}");

}