fn main() {
    // call function 
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char){
    // display message
    println!("The measurement is: {value}{unit_label}");
}
