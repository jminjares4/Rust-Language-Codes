fn main() {
    
    // construct tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);
    // store each value of tuple in a variable

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // display values
    println!("Value 0: {five_hundred}");
    println!("Value 1: {six_point_four}");
    println!("Value 2: {one}");
}
