fn main(){
    let mut x = 4;
    println!("x is: {}", x);
    //x = "Hello!"; //this can't be done in Rust but can be done in Python.
    let x = 5;
    println!("x is: {}", x);

    //u8 = 0 to 2^8-1 => 0 to 255
    //i8 = -2^7 to +2^7+1 => -128 to 127
    //u => unassigned => means taking only positive numbers
    //i => assigned => means taking negative as well as positive numbers

    //i8 + u8 => can't happen in rust

    // if u8 = 255 and u8 = 1, when we add both of these numbers then it will become 256 which will cause overflow condition.
}