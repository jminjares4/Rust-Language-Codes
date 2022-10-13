fn main() {
    // create a mutable variable counter 
    let mut count = 0;

    // label loop
    'counting_up: loop {
        // display current count
        println!("count = {count}");

        // create mutable varaible 
        let mut remaining = 10;

        loop {
            //display remaining
            println!("remaining = {remaining}");
            // check is remaining is 9
            if remaining == 9 {
                break;
            }
            // check if count is 1
            if count == 2 {
                // break counting up loop
                break 'counting_up;
            }
            // decrement by 1
            remaining -= 1;
        }
        // increment by 1
        count += 1;
    }
    // display count 
    println!("End count = {count}");
}
