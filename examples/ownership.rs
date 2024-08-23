
fn main() {
    // Ownership
    // 1. Each value in rust has it's owner and it's a variable
    // 2. There can only be ONE owner at a time
    // 3. When owner goes out of scope the value will be dropped instantly

    {
        // the 'name' is not defined till this point
        let _name = "rust";
        // now the 'name' is defined and known to the memory
    }
    // 'name' is now no more it's ownership is lost after it's out of the scope

    // There is a difference between strings on how we define them
    let _language = "rust";
    // here we know that it's a string which is immutable and it's value is fixed.
    // so it's stored in binary directly

    let _type: String = String::from("low level");
    // now we created a dynamic string variable _type who's size is dynamically
    // taken based on the value and this is stored in heap 

    // Copy / Move / Clone
    
    let _x = 10;
    let _y = _x;
    // here the x stores 10 and y stores the copy of x which is 10; the value is copied
    
    let _new = String::from("i am new to Rust");
    let _old = _new;
    // here the _new stores {ptr, len, capacity} in the heap where pointer is pointing to actual value's 0th index
    // when copied to _old the _old will be pointing to the heap of _new but, the _new will be invalidated.
    // basically the _new will be MOVED to _old in the memory
    // when we try to access _new we will get error saying 'shallow copy'
    // to actually copy from _new to _old what we do is CLONE the value from _new to _old
    // something like this let _old = _new.clone() 
    // if we can bear the cost of performance loss
}