enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
   let localhost: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);

//    enum Option<T> {
//        Some(T),
//        None,
//    }

   let some_number: Option<i32> = Some(5);
   let some_string: Option<String> = Some(String::from("hello"));
   let absent_number: Option<i32> = None;

   println!("some_number = {:?}, some_string = {:?}, absent_number = {:?}", some_number, some_string, absent_number);

   let x: i32 = 5;
    let y: Option<i32> = Some(5); // None; //Try changing this to None to see the difference
    let sum: i32 = x + y.unwrap_or(0);
    println!("sum = {}", sum);

    let coin: Coin = Coin::Dime;
    println!("value in cents = {}", value_in_cents(coin));

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);
    println!("six = {:?}, none = {:?}", six, none);
    
}
