fn main() {
    // default cargo new output
    println!("Hello, world!");

    another_function(35);
}

// create another function
fn another_function(x: i32){
    // display message
    println!("The value of x is: {x}");
}
