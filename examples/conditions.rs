

fn main() {
    
    // Conditions
    // Core conditional operators
    // > , < , >= , <= , != , ==
    let _condition1:bool = 4 > 3; //true
    let _condition2: bool = 5 != 5; //false
    // Strictly the operands on RHS and LHS should be of same type as usual!
    // if different types to be compared better is to typecast it using -> let var = (value as <type>);
    let _condition3: bool = (2 as f32) > 1.4; // true
    // Logical conditions ~ Compound conditions (and, or, not)
    // In Rust its syntaxed as &&, ||, !

    // The Logical AND or && operator will check whether both the operands are true or not and will return true is both are true
    let _logical1: bool = _condition1 && _condition3;
    println!("The logical && of condition1 and condition3 is : {}", _logical1); // true

    // The Logical OR or || operator will check either of the operands are true or not and returns the bool
    let _logical2: bool = _condition1 || _condition2;
    println!("The logical || of condition1 and condition2 is : {}", _logical2); // true since one of the operand(LHS) is true

    // The Logical NOT or ! operator it is also called as Negation, Complement, which will basically flip the bool to opposite of it
    let _logical3: bool = !_condition1; 
    println!("The logical ! of condition1  : {}", _logical3); // false since the condition1 is true and it's opposite is false.

    // Important to remember if applying compound or more than one Logical operator is the Operator Precedence
    // The ! (NOT) should be first, followed by && and finally ||
    // if you're not wrapping in a parentheses ()
}
