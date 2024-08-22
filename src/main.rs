use std::io;

// Some shitty proj with what I've learnt till now :)

fn main() {
    let input: i32 = input_age();
    validate_age(input);
}

fn validate_age(input: i32) {
    if (input == 20 || input >= 20) && input < 25 {
        println!(
            "Your age is {} and you are not a TEEN anymore you're an early adult :)",
            input
        );
    } else if input >= 25 && input < 60 {
        println!("Your age is {} and you are a fully mature Adult!", input);
    } else if input >= 60 && input <= 110 {
        println!("You are a senior citizen! Great")
    } else {
        println!("Invalid age!");
    }
}

fn input_age() -> i32 {
    // Let's take an input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read input");

    // Let's typecast the input to int
    let input: i32 = input.trim().parse().unwrap();

    // Let's return back the value
    return input;
}
