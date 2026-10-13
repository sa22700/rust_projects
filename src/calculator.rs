use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::thread;
use std::time::Duration;

static STOP: AtomicBool = AtomicBool::new(false);

fn read_input(rx: &Receiver<String>) -> Option<String> {
    while !STOP.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(input) => return (!STOP.load(Ordering::SeqCst)).then_some(input),
            Err(RecvTimeoutError::Disconnected) => return None,
            Err(RecvTimeoutError::Timeout) => {}
        }
    }
    None
}

fn add(first: i32, second: i32) -> i32 {
    first + second
}

fn subtract(first: i32, second: i32) -> i32 {
    first - second
}

fn multiply(first: i32, second: i32) -> i32 {
    first * second
}

fn divide(first: i32, second: i32) -> f32 {
    first as f32 / second as f32
}

pub fn calc() {
    STOP.store(false, Ordering::SeqCst);
    ctrlc::set_handler(|| {
        STOP.store(true, Ordering::SeqCst);
    }).expect("error");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap_or(0) == 0
            || tx.send(input).is_err() {
            break;
        }
    });
    loop {
        println!("--- Calculator ---\n1. Add\n2. Substract\n3. Multiply\n4.Divide\n-----------------\nChoose an option (1-4):");
        let Some(input) = read_input(&rx) else { return; };
        if input.trim().parse::<i32>().is_ok() {
            let change_int: i32 = input.trim().parse().unwrap();
            if change_int <= 0 || change_int >= 5 {
                println!("Use number between 1-4")
            } else {
                println!("Add first number");
                let Some(first_input) = read_input(&rx) else { return; };
                let first_int: i32 = first_input.trim().parse().expect("please give me correct string number!");
                println!("Add second number");
                let Some(second_input) = read_input(&rx) else { return; };
                let second_int: i32 = second_input.trim().parse().expect("please give me correct string number!");
                if change_int == 1 {
                    let answer = add(first_int, second_int);
                    println!("\n{first_int} + {second_int} = {answer}")
                }
                if change_int == 2 {
                    let answer = subtract(first_int, second_int);
                    println!("\n{first_int} - {second_int} = {answer}")
                }
                if change_int == 3 {
                    let answer = multiply(first_int, second_int);
                    println!("\n{first_int} x {second_int} = {answer}")
                }
                if change_int == 4 {
                    if second_int == 0 {
                        println!("Number cannot be 0 in divide operator")
                    } else {
                        let answer = divide(first_int, second_int);
                        println!("\n{first_int} / {second_int} = {answer}");
                    }
                }
            }
        }
        else {
            println!("Enter valid number");
        }
    }
}