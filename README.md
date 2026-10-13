This is a simple Rust application that prints "Hello World" to the terminal.

I learned how to create and run a Rust project with Cargo and how the Rust project structure works. I also learned that build artifacts are stored in the target directory and should not be committed to a Git repository.

-----------------------------------------------------------------------------------------------------------------

Simple Calculator in Rust

A simple console-based calculator written in Rust.

The calculator supports:

Addition

Subtraction

Multiplication

Division

The program runs in a loop until it is stopped with Ctrl-C.

-----------------------------------------------------------------------------------------------------------------

A simple Rust program that extracts severity levels from log messages using string slices.

The function returns the original log message if the severity level cannot be found.

I learned how to use &str, string slicing, borrowing, and references without creating unnecessary String values.

-----------------------------------------------------------------------------------------------------------------

Library Loan in Rust

A simple Rust program that models a library book loan.

The program uses a BookLoan struct to store the book title, borrower, and remaining loan days.

It includes methods to create a new loan, print loan information, and reduce the remaining days. The loan starts with 14 days and cannot go below zero.

-----------------------------------------------------------------------------------------------------------------

## AI usage

I used ChatGPT to help me understand Rust concepts such as string slices, borrowing, ownership, and the use of &str. I also used it to review my code, assist with implementation, and explain why certain parts work.

I learned how string slices reference parts of an existing string without creating a new String, and how Rust's borrowing rules affect references.

In the Library Loan assignment, I used ChatGPT to help me understand structs, impl blocks, associated functions, and the differences between &self and &mut self.

I also learned how to use Self, convert &str values into String values, and prevent integer underflow using saturating_sub().
