# Closures

Doc: https://doc.rust-lang.org/book/ch13-01-closures.html

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined.

## Capturing the environment

```rs
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
```

The store defined in main has two blue shirts and one red shirt remaining to distribute for this limited-edition promotion. We call the `giveaway` method for a user with a preference for a red shirt and a user without any preference.The store defined in main has two blue shirts and one red shirt remaining to distribute for this limited-edition promotion. We call the giveaway method for a user with a preference for a red shirt and a user without any preference.The store defined in main has two blue shirts and one red shirt remaining to distribute for this limited-edition promotion. We call the giveaway method for a user with a preference for a red shirt and a user without any preference.

In the giveaway method, we get the user preference as a parameter of type Option<ShirtColor> and call the unwrap_or_else method on user_preference. The unwrap_or_else method on Option<T> is defined by the standard library. It takes one argument: a closure without any arguments that returns a value T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor). If the Option<T> is the Some variant, `unwrap_or_else` returns the value from within the Some. If the Option<T> is the None variant, `unwrap_or_else` calls the closure and returns the value returned by the closure.

We specify the closure expression `|| self.most_stocked()` as the argument to unwrap_or_else. This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical pipes). The body of the closure calls self.most_stocked(). We’re defining the closure here, and the implementation of `unwrap_or_else` will evaluate the closure later if the result is needed.

Running this code prints the following:

```$ cargo run
   Compiling shirt-company v0.1.0 (file:///projects/shirt-company)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/shirt-company`
The user with preference Some(Red) gets Red
The user with preference None gets Blue
```
## Inferring and Annotating Closure Types

Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do. Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables. 

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The first line shows a function definition and the second line shows a fully annotated closure definition. In the third line, we remove the type annotations from the closure definition. In the fourth line, we remove the brackets, which are optional because the closure body has only one expression. These are all valid definitions that will produce the same behavior when they’re called. The `add_one_v3` and `add_one_v4` lines require the closures to be evaluated to be able to compile because the types will be inferred from their usage.

## Capturing References or Moving Ownership

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. 

```rs
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}
```

output:
```
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

Next, in Listing 13-5, we change the closure body so that it adds an element to the list vector. The closure now captures a mutable reference.

```rs
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");
}
```
Output:
```
$ cargo run
   Compiling closure-example v0.1.0 (file:///projects/closure-example)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/closure-example`
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
```
Note that there’s no longer a `println!` between the definition and the call of the borrows_mutably closure: When borrows_mutably is defined, it captures a mutable reference to list. We don’t use the closure again after the closure is called, so the mutable borrow ends. Between the closure definition and the closure call, an immutable borrow to print isn’t allowed, because no other borrows are allowed when there’s a mutable borrow. Try adding a `println!` there to see what error message you get!

If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the `move` keyword before the parameter list.

```rs
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}
```

We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the list. In Listing 13-4, the closure only captured list using an immutable reference because that’s the least amount of access to list needed to print it. In this example, even though the closure body still only needs an immutable reference, we need to specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition. If the main thread performed more operations before calling join on the new thread, the new thread might finish before the rest of the main thread finishes, or the main thread might finish first. If the main thread maintained ownership of list but ended before the new thread and drops list, the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved into the closure given to the new thread so that the reference will be valid. Try removing the move keyword or using list in the main thread after the closure is defined to see what compiler errors you get!

Errors I got:
```
error[E0277]: expected a `FnOnce()` closure, found `()`
   --> src/main.rs:7:19
    |
  7 |     thread::spawn(println!("From thread: {list:?}"))
    |     ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnOnce()` closure, found `()`
    |     |
    |     required by a bound introduced by this call
    |
    = help: the trait `FnOnce()` is not implemented for `()`
    = note: wrap the `()` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `spawn`error[E0277]: expected a `FnOnce()` closure, found `()`
   --> src/main.rs:7:19
    |
  7 |     thread::spawn(println!("From thread: {list:?}"))
    |     ------------- ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an `FnOnce()` closure, found `()`
    |     |
    |     required by a bound introduced by this call
    |
    = help: the trait `FnOnce()` is not implemented for `()`
    = note: wrap the `()` in a closure with no arguments: `|| { /* code */ }`
note: required by a bound in `spawn`
```

## Moving Captured Values Out of Closures

A closure body can do any of the following: Move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

- `FnOnce`: applies to closures that can be called once. All closures implement at least this trait because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits because it can only be called once.
- `FnMut`: applies to closures that don’t move captured values out of their body but might mutate the captured values. These closures can be called more than once.
- `Fn`: applies to closures that don’t move captured values out of their body and don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

Because all closures implement FnOnce, `unwrap_or_else` accepts all three kinds of closures and is as flexible as it can be.

```rs
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}
```

