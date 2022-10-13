fn main() {
    // create mutable counter variable
    let mut counter = 0;

    // store loop expression return to result
    let result = loop {
        // increment counter by 1
        counter += 1;
        // check if counter is 10
        if counter == 10 {
            // break loop and return counter * 2
            break counter * 2;
        }
    };

    // display value
    println!("The result is {result}");
}
