use rand::Rng;
use std::cmp::Ordering;
use std::io; //using standard input and output // using rand //use match
fn main() {
    //print to the terminal
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    // infinite loop
    loop {
        println!("Please input your guess.");
        // 'let' lets us to create variables
        //  mut' states that the variable will be mutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //trim -> remove any white spaces and \r\n,
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        println!("You have guessed: {}", guess);

        // check secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
