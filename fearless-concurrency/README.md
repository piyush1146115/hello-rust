# Fearless Concurrency

doc: https://doc.rust-lang.org/book/ch16-00-concurrency.html

Handling concurrent programming safely and efficiently is another of Rust’s major goals. Concurrent programming, in which different parts of a program execute independently, and parallel programming, in which different parts of a program execute at the same time, are becoming increasingly important as more computers take advantage of their multiple processors. 

Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges to be solved with different methods. Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and concurrency problems! By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors. 

Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages because a higher-level language promises benefits from giving up some control to gain abstractions. However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

Here are the topics we’ll cover in this chapter:

- How to create threads to run multiple pieces of code at the same time
- Message-passing concurrency, where channels send messages between threads
- Shared-state concurrency, where multiple threads have access to some piece of data
- The Sync and Send traits, which extend Rust’s concurrency guarantees to user-defined types as well as types provided by the standard library