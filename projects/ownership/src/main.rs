fn main() {
    
    let x: i32 = 5;
    let y: i32 = x; // Copy (shallow copy) 
    println!("x = {}, y = {}", x, y);

    let s1: String = String::from("hello");
    // let s2: String = s1; // Move (not shallow copy)
    let s2: String = s1.clone(); // Deep copy

    println!("{}, world!", s2); //this works
    println!("{}, world!", s1); //this does not work with move, but works with clone

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    // println!("{}", s); // this does not work

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still
                   // use x afterward
    println!("{}", x); // this works


    let s3 = gives_ownership(); // gives_ownership moves its return
                                                  // value into s3
    println!("{}", s3);
    let s4 = String::from("hello"); // s4 comes into scope
    let s5 = takes_and_gives_back(s4); // s4 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s5
    // println!("{}", s4); // this does not work
    println!("s5 = {}", s5);

    let s6 = String::from("hello");
    let len = calculate_length(&s6);
    println!("The length of '{}' is {}.", s6, len); // this works only because we used a reference, this does not work if we use ownership
    println!("The length is {}.", len);


    let mut s7: String = String::from("hello");

    let r1: &mut String = &mut s7;
    // let r2: &mut String = &mut s7; // this does not work because we can only have one mutable reference to a particular piece of data in a particular scope
    println!("{}, world!", r1);
    // println!("{}, world!", r2);

    let mut s8: String = String::from("hello");

    let r1: &String = &s8; // no problem
    let r2: &String = &s8; // no problem
    println!("{}, world!", r1);
    println!("{}, world!", r2);
    // let r3: &mut String = &mut s8; // BIG PROBLEM
    // println!("{}, world!", r3);

    // The Rules of references
    // 1. At any given time, you can have either one mutable reference or any
    //    number of immutable references.
    // 2. References must always be valid.
}

// fn dangle() -> &String { // this does not work because we are returning a reference to a value that will go out of scope
//     let s = String::from("hello"); // s comes into scope

//     &s // we return a reference to the String, s
// } // s goes out of scope, and is dropped. Its memory goes away.

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed.   

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) ->  usize {
    let length = s.len(); // len() returns the length of a String

    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}