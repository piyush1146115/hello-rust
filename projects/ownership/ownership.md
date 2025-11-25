# Understanding Ownership in Rust

Doc link: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html 

## Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped


The Rules of References
- At any given time, you can have wither one mutable reference or any number of immutable reference
- References must always be valid
- 

## Pros of the ownership model

Pros:
- Control over memory
- Error free
- Faster runtime
- Smaller program size

Cons:
- Slower write time. Learning curve


## Stack and Heap
- Stack are fixed sized memory block. It can't grow or shrink during runtime
- Stack has stack frames which are created for every function that executes
- The size of the stack frames are calculated at compile time
- Stack stores the function execution calls
- Storing and accesing data on stack are faster than heap
- Stack executes and stores the stack frames in LIFO order

- On the other hand, Heap is less organized than Stack
- Heap can grow and shrink in runtime
- It can be large amount of data and we control the lifetime of the data
- Pushing values to the stack is fater than allocating the memory in the heap
- Passing in references as function parameters is called borrowing. We are borrowing the values, but not actually taking the ownership of it
- References are immutable by default. We can not borrow the value as mutable

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. Once you understand ownership, you won’t need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

## Memory and Allocation

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here’s a version of our scope example from Listing 4-1 using a String instead of a string literal.

```
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

```

There is a natural point at which we can return the memory our String needs to the allocator: when s goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.