fn main() {
    // integer of size 8 bits 
    let i_eight:i8 = 0b0100_1010;
    let u_eight:u8 = b'A';

    // integer of size 16 bits 
    let i_sixteen:i16 = 32000;
    let u_sixteen:u16 = 0xFFAC;

    // integer of size 32 bits 
    let i_thirtytwo:i32 = 135_000;
    let u_thirtytwo:u32 = 0xFFFF_ABCD;

    // integer of size 64 bits 
    let i_sixtyfour:i64 = 1024124;
    let u_sixtyfour:u64 = 0xFFFF_FFFF_FFFA_DEAD;

    // display values 
    println!("The value of i8 is: {i_eight}");
    println!("The value of u8 is: {u_eight}");
    println!("The value of i16 is: {i_sixteen}");
    println!("The value of u16 is: {u_sixteen}");
    println!("The value of i32 is: {i_thirtytwo}");
    println!("The value of u32 is: {u_thirtytwo}");
    println!("The value of i64 is: {i_sixtyfour}");
    println!("The value of u64 is: {u_sixtyfour}");

}
