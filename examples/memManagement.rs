

fn main() {
    
    // Memory management
    // Stacks and Heap are crucial in rust for memory management
    
    
    // Stack: Last person IN will be kicked out first
    // Heap: Not the same heap as heap data structure a bit diff
    // Storing something on a stack is slower because we've to find a space first then store it
    let _i = 10;
    let _j = String::from("Hello, world");
    // in the above implementation
    // i variable is also holding a int j variable is holding the string
    // The i variable string is immutable. it is fixed. we can't change the
    // value or size or type of that variable.
    // and it stores the value and the variable name i in a stack.
    // where as in the j variable, the obj String is being stored in a stack AND
    // with the value as pointer pointing to where the actual "Hello, world" is stored,
    // the actual value "Hello, world" is being stored in a heap. Which is completly different
    // from stack data structure.
    // So stack is storing the address of where the actual value is stored and
    // Heap is storing the actual value in it.
}

