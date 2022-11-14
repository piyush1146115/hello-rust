fn main(){
    // immutable variables
    let number = 12;
    let other_number = 53;
    print!("{}\n", number + other_number);

    // Mutable Variables
    let mut number = 12;
    print!("{}\n", number);
    number = 53;
    print!("{}\n", number);
}