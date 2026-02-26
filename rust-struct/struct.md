# Using Structs to Structure Related Data

Ref:
-  https://doc.rust-lang.org/book/ch05-00-structs.html
- https://doc.rust-lang.org/book/ch05-01-defining-structs.html 

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes. In this chapter, we’ll compare and contrast tuples with structs to build on what you already know and demonstrate when structs are a better way to group data.

## Defining and Instantiating Structs

Structs are similar to tuples, discussed in “The Tuple Type” section, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: You don’t have to rely on the order of the data to specify or access the values of an instance.

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

```

To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We create an instance by stating the name of the struct and then add curly brackets containing key: value pairs, where the keys are the names of the fields and the values are the data we want to store in those fields.

```rs
fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}
```

In other words, the struct definition is like a general template for the type, and instances fill in that template with particular data to create values of the type.

To get a specific value from a struct, we use dot notation. For example, to access this user’s email address, we use user1.email. If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.

```rs
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable. As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.

### Using the Field Init Shorthand

Following listing  shows a build_user function that returns a User instance with the given email and username. The active field gets the value true, and the sign_in_count gets a value of 1.

```rs
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

```

Because the parameter names and the struct field names are exactly the same in the above Listing, we can use the field init shorthand syntax to rewrite build_user so that it behaves exactly the same but doesn’t have the repetition of username and email:

```rs
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

```

### Creating Instances with Struct Update Syntax

First, in the following Listing, we show how to create a new User instance in user2 in the regular way, without the update syntax. We set a new value for email but otherwise use the same values from user1.

```rs
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Using struct update syntax, we can achieve the same effect with less code, as shown in the following. The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.

```rs
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

The ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition. In this example, we can no longer use user1 after creating user2 because the String in the username field of user1 was moved into user2. If we had given user2 new String values for both email and username, and thus only used the active and sign_in_count values from user1, then user1 would still be valid after creating user2. Both active and sign_in_count are types that implement the Copy trait.

### Creating Different Types with Tuple Structs

Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

To define a tuple struct, start with the struct keyword and the struct name followed by the types in the tuple. For example, here we define and use two tuple structs named Color and Point:

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

### Defining Unit-Like Structs

You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to (). Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself. 

```rs
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

To define AlwaysEqual, we use the struct keyword, the name we want, and then a semicolon. No need for curly brackets or parentheses! Then, we can get an instance of AlwaysEqual in the subject variable in a similar way: using the name we defined, without any curly brackets or parentheses.

## Ownership of Struct Data

In the User struct definition, we used the owned String type rather than the &str string slice type. This is a deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.

It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes.Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.