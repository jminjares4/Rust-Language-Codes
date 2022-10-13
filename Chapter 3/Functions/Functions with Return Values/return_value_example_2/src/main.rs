fn main() {
    // call plus one function
    let x = plus_one(5);
    // display message 
    println!("The value of x is: {x}");
}

fn plus_one(x:i32) -> i32 {
    x + 1 // no semicolon as it will return it implicitly
}