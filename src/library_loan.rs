
pub struct BookLoan {
    title: String,
    borrower: String,
    days_remaining: u32,
}

impl BookLoan {
    pub fn new(title: &str, borrower: &str) -> Self {
        Self {
            title: String::from(title),
            borrower: String::from(borrower),
            days_remaining: 14,
        }
    }

    pub fn summary(&self) {
        println!("{} is borrowed by {} for {} more days.",
            self.title, self.borrower, self.days_remaining
        );
    }

    pub fn pass_day(&mut self) {
        self.days_remaining = self.days_remaining.saturating_sub(1);
    }
}
