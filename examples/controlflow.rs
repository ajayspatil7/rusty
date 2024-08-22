

fn main() {
    // Control flow
    // 1. IF -- ELSE
    let food: &str = "Shit";

    if food == "Cookie" {
        println!("You can eat the {}", food);
    } else {
        println!("Don't eat the {}", food);
    }

    // 2. ElSE IF
    let age: i8 = 17;

    if age > 18 {
        println!("You can legally vote");
    } else if age == 18  {
        println!("You can legally vote since you are {}", age);
    } else {
        println!("Oops! You can't vote :( your age is below 18");
    }

}
