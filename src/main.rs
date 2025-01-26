use std::io;
use std::thread;

fn main() {
    loop {
        println!("Calculator");
        println!("Main Menu");
        println!("1. Calculate factorial");
        println!("2. Calculate square root");
        println!("3. Calculate cubic root");
        println!("4. Calculate log10");
        println!("5. Exit");
        let mut option = String::new(); // Creates a string variable to catch user's option in the menu
        io::stdin()
            .read_line(&mut option) //Stores the user's input in a variable
            .expect("Failed to read line"); //Expect is used to encode error-handling information.
        let user_choice: u64 = option.trim().parse().expect("Please type a number!"); // The string variable "option" is converted into a integer
        if user_choice == 1 {
            //The right function will be called according to the user's input
            input();
        } else if user_choice == 2 {
            input_root(2);
        } else if user_choice == 3 {
            input_root(3);
        } else if user_choice == 4 {
            input_log();
        } else if user_choice == 5 {
            return;
        }
    }
}

fn input() {
    // The function input is declared
    println!("\nType a number");
    let mut num_input = String::new(); //Creates a new string variable
    io::stdin()
        .read_line(&mut num_input) //Stores the user's input in a variable
        .expect("Failed to read line"); //Expect is used to encode error-handling information.
    let num_input: u64 = num_input.trim().parse().expect("Please type a number!"); // Converts a string into an integer
    let outcome: u64 = factorial(num_input); //Calls the factorial function and sends the user's input
    println!("The factorial of {} is: {}\n", num_input, outcome);
    thread::sleep_ms(3000); // Thread puts the current thread to sleep. This line allows the outcome to show for 2.5 seconds
}

fn factorial(number: u64) -> u64 {
    if number <= 1 {
        // If number is 1 or less, the function will return 1
        1
    } else {
        //If number is greater then 1, the recursion will be executed until number is 1
        number * factorial(number - 1) // The function factorial is called and number is reduced by one
    }
}

fn input_root(number: u64) {
    println!("\nType a number");
    let mut num_input = String::new(); //Creates a new string variable
    io::stdin()
        .read_line(&mut num_input) //Stores the user's input in a variable
        .expect("Failed to read line"); //Expect is used to encode error-handling information.
    let num_input: f64 = num_input.trim().parse().expect("Please type a number!"); // Converts a string into an float
    if number == 2 {
        square(num_input); //If the function was called from square root, it will call the square function
    } else if number == 3 {
        cubic(num_input); //If the function was called from cubic root, it will call the cubic function
    }
    thread::sleep_ms(3000); // Thread puts the current thread to sleep. This line allows the outcome to show for 2.5 seconds
}

fn square(number: f64) {
    if number < 0.0 {
        // If the number received is lower than zero, it will show an error message
        println!("Please enter a positive number");
    } else {
        let square_root: f64 = number.sqrt(); //If the number is greater than zero, the square root is calculated
        println!("The square root of {} is: {}\n", number, square_root);
    }
}

fn cubic(number: f64) {
    if number < 0.0 {
        // If the number received is lower than zero, it will show an error message
        println!("Please enter a positive number");
    } else {
        let cubic_root: f64 = number.powf(1.0 / 3.0); //If the number is greater than zero, the cubic root is calculated
        println!("The cubic root of {} is: {}\n", number, cubic_root);
    }
}

fn input_log() {
    println!("\nType a number");
    let mut num_input = String::new(); //Creates a new string variable
    io::stdin()
        .read_line(&mut num_input) //Stores the user's input in a variable
        .expect("Failed to read line"); //Expect is used to encode error-handling information.
    let num_input: f64 = num_input.trim().parse().expect("Please type a number!"); // Converts a string into an float
    log(num_input);
    thread::sleep_ms(3000); // Thread puts the current thread to sleep. This line allows the outcome to show for 2.5 seconds
}

fn log(number: f64) {
    if number < 0.0 {
        // If the number received is lower than zero, it will show an error message
        println!("Please enter a positive number");
    } else {
        let log: f64 = number.log10(); //If the number is greater than zero, the log10 of the number is calculated
        println!("The Log10 of {} is: {}\n", number, log);
    }
}
