#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // single line
    println!("rect1 is {:?}", rect1);
    // pretty outptu
    println!("rect1 is {:#?}", rect1);
}