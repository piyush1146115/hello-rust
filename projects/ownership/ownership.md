# Understanding Ownership in Rust

## Ownership Rules

First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:
- Each value in Rust has a variable, that's called its owner
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
