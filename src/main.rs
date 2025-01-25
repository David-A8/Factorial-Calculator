use std::io;

fn main() {
    println!("Factorial Calculator");
    input();
}

fn input() {
    println!("Type a number");
    let mut input = String::new(); //Creates a new string variable
    io::stdin()
        .read_line(&mut input) //Stores the user's input in a variable
        .expect("Failed to read line"); //Expect is used to encode error-handling information.
    let input: u64 = input.trim().parse().expect("Please type a number!"); // Converts a string into an float
    let outcome: u64 = factorial(input);
    println!("The factorial of {} is: {}", input, outcome);
}

fn factorial(number: u64) -> u64 {
    if number <= 1 {
        1
    } else {
        number * factorial(number - 1)
    }
}
