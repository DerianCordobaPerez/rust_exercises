use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let random_number = rand::thread_rng()
        .gen_range(1..=100);
    
    loop {
        let mut number = String::new();

        println!("Your random number is: {random_number}");

        println!("Please insert a number.");

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: u32 = match number.trim()
            .parse() {
                Ok(number) => number,
                Err(_) => continue,
            };

        // is_correct_number is a boolean value that indicates if the number is correct.
        // also, it can be renamed to should_break_loop or something like that.
        let (message, is_correct_number) = compare_numbers_and_return_message(number, random_number);
        println!("{message}");
        if is_correct_number {
            break;
        }
    }
}

/// Compare numbers and return a message with a boolean value indicating if the number is correct.
/// The message is a string with the following values:
/// - "Too small!"
/// - "Too big!"
/// - "You win!"
/// The boolean value is true if the number is correct, otherwise false.
/// The function receives two u32 values, the number to compare and the random number.
/// The function returns a tuple with the message and the boolean value.
fn compare_numbers_and_return_message(number: u32, random_number: u32) -> (String, bool) {
    match number.cmp(&random_number) {
        Ordering::Less => (String::from("Too small!"), false),
        Ordering::Greater => (String::from("Too big!"), false),
        Ordering::Equal => (String::from("You win!"), true),
    }
}
