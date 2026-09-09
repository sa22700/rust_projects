use std::io;

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
    while  true {
        println!("--- Calculator ---\n1. Add\n2. Substract\n3. Multiply\n4.Divide\n-----------------\nChoose an option (1-4):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to readline");
        if input.trim().parse::<i32>().is_ok() {
            let change_int: i32 = input.trim().parse().unwrap();
            if change_int <= 0 || change_int >= 5 {
                println!("Use number between 1-4")
            } else {
                println!("Add first number");
                let mut first_input = String::new();
                io::stdin().read_line(&mut first_input).expect("Failed to readline");
                let first_int: i32 = first_input.trim().parse().expect("please give me correct string number!");
                println!("Add second number");
                let mut second_input = String::new();
                io::stdin().read_line(&mut second_input).expect("Failed to readline");
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