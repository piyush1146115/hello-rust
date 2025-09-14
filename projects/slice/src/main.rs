fn main() {
    let mut s: String = String::from("hello world");
    let hello: &str = &s[0..5]; // first 5 characters
    let world: &str = &s[6..11]; // last 5 characters


    let word: &str = first_word(&s); // word will get the first word
    println!("the first word is: {}", word);
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now
               // what we call a dangling reference
    // println!("the first word is: {}", word); // this will cause a compile-time error

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3]; // this is
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    // for (i: usize, &item: u8) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return &s[0..i];
    //     }
    // }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return the slice from start to the index of the space
        }
    }

    &s[..] // if no space is found, return the whole string
}