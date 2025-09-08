fn main() {
    println!("Hello, world!");
    let result = my_function(5, 10);
    println!("The result is: {}", result);

    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter > 10 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Adding {} and {}", x, y);
    x + y
}