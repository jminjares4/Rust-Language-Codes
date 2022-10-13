// use predule standard library
use std::io;

fn main() {
    // create an array of 5 integers 
    let a = [1, 2, 3, 4, 5];

    // display message 
    println!("Please enter an array index.");

    // create a index variable
    let mut index = String::new();

    // get user input
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // check input
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // store array at index
    let element = a[index];

    // display value
    println!("The value of the element at index {index} is: {element}");
}
