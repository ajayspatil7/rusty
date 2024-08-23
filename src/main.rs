

fn main() {
    // References
    // Let's say we have a scenerio where we are supposed to copy a value from a var as pass it to
    // a function as a parameter and that function returns a string and number which you want to store it somewhere

    let new = String::from("hello");    
    let len = calculate_str_len(&new);
    println!("Str {} and Len {}", new, len);

    // the above code works perfectly fine but 
    // our function returns string and we're storing
    // what if we were to store it when our function didn't return anything
    // we could use the _new only but, string copy is not allowed and
    // we can't risk the performance. 
    
    // We use references!
    // We make our calculate_str_len() function to accept the reference of a string
    // rather than the string itself.
    // Then, we would not need to return the text since we are taking it from the new var itself
    // Then, while passing the value to the text: parameter we pass it as a reference not as a raw string.
    // Visually : text(pointer) -> new -> "hello" which is in the heap
    // REFERENCES DO NOT TAKE ANY OWNERSHIP


}


fn calculate_str_len(text: &String) -> usize{
    let length = text.len();
    return length;
}