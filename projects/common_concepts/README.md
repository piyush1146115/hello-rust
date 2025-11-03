# Common concepts in Rust

Video tutorial: https://youtu.be/2V0JaMVjzws?si=Po_ZKCI0eVlRQcvG 
Doc link: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html


## Variables and Mutability

- Rust variables are immutable by default
- When a variable is immutable, once a value is bound to a name you can't change value
- Although variables are immutable by default, you can make them mutable by adding mut in front of the variable name. Adding mut also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.
- Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables. Rust’s naming convention for constants is to use all uppercase with underscores between words


```
fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

```

### Shadowing

you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. 

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

```

Output:
```
The value of x in the inner scope is: 12
The value of x is: 6
```

Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.


The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

```
    let spaces = "   ";
    let spaces = spaces.len();

```

The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name.

## Data Types

- Scalar data types and compound data types
- Rust has 4 main scalar data types: Integers, Floating-point numbers, Booleans, Character

```rs
//integers
let a: i32 = 98_222; //Decimal
let b: i32 = 0xff; // Hex
let c: i32 = 0o77; // Octal

let f: u8 = 255;


// Floating point numbers
let f: f64 = 2.0;
let g: f32 = 3.0;

// Addition
let sum: i32 = 5 + 10;
// Subtraction
let difference: f64 = 95.5 - 4.3;
// Multiplication
let product: i32 = 4*30;

// Booleans
let t: bool = true;
let f: bool = false;

// Character
let c: char = 'z';
let z: char = 'Z';

```


### Compound types

These are data types which represent a group of values.

```rs
let tup: (&str, i32) = ("Let' get Rusty!", 100_000);
let (channel: &str, sub_count: i32) = tup;
let sub_count: i32 = tup.1;


let error_codes: [i32; _] = [300, 404, 500];
let not_found: i32 = error_codes[1];


```

- In Rust, a piece of code is either a statement or a expression. A statement doesn't return a value, while a expression returns a value after execution
