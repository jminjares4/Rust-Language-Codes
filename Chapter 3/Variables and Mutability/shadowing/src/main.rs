fn main() {
    // set x to 5
    let x = 5;
    // overshadow x 
    let x = x + 1; // x:5, x + 1 = 6
    // create a inner scope 
    {
        // overshadow x 
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {x}");

}
