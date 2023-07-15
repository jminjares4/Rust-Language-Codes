#![allow(unused)]
fn main() {
    // print character
    for c in "Зд".chars() {
        println!("{c}");
    }
    // alternative, print bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }    
}
