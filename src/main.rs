mod calculator;
mod slice_string;
mod library_loan;
use crate::library_loan::BookLoan;
use crate::slice_string::extract_severity;
fn main() {
    calculator::calc();
    println!();
    let log = String::from("[WARN] Disk space reaching 90%");
    let severity = extract_severity(&log);
    println!("Severity: {severity}");
    let mut loan = BookLoan::new("The Rust Book", "Iines");
    loan.summary();
    loan.pass_day();
    loan.pass_day();
    loan.pass_day();
    loan.summary();

}