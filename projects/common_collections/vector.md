# Storing Lists of Values with Vectors

Ref: https://doc.rust-lang.org/book/ch08-01-vectors.html#storing-lists-of-values-with-vectors

Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory. Vectors can only store values of the same type. They are useful when you have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

## Creating a New Vector

```rs
fn main() {
    let v: Vec<i32> = Vec::new();
}
```

Vectors are implemented using generics; the `Vec<T>` type provided by the standard library can hold any type.

Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it.

```rs
fn main() {
    let v = vec![1, 2, 3];
}
```
To create a vector and then add elements to it, we can use the push method:

```
fn main() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}
```

## Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or by using the get method. In the following examples, we’ve annotated the types of the values that are returned from these functions for extra clarity.

```rs
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
```
When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.



```rs
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

```

When we run this code, the first [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules (covered in Chapter 4) to ensure that this reference and any other references to the contents of the vector remain valid. Recall the rule that states you can’t have mutable and immutable references in the same scope. That rule applies in Listing 8-6, where we hold an immutable reference to the first element in a vector and try to add an element to the end. This program won’t work if we also try to refer to that element later in the function.

```rs
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");
}
```

Compiling this code will result in this error:

```rs
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {first}");
  |                                      ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` (bin "collections") due to 1 previous error
```
The code in Listing 8-6 might look like it should work: Why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

## Iterating Over the Values in a Vector

To access each element in a vector in turn, we would iterate through all of the elements rather than use indices to access one at a time. Listing 8-7 shows how to use a for loop to get immutable references to each element in a vector of i32 values and print them.

```rs
fn main() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
}
```

We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The for loop in Listing 8-8 will add 50 to each element.

```rs
fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

```