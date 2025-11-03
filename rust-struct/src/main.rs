struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {

    let mut user1: User = User {
        email: String::from("bogdan@gmail.com"),
        username: String::from("bogdan"),
        active: true,
        sign_in_count: 1,
    };  

    let name: String = String::from("alice");
    user1.username = name;
    println!("username = {}", user1.username);

    let user2: User = build_user(
        String::from("james@mail.com"),
        String::from("james")
    );

    println!("user2 email = {}, username = {}", user2.email, user2.username);

    let user3: User = User {
        email: String::from("kyle@mail.com"),
        username: String::from("kyle123"),
        ..user2 // struct update syntax
    };
    println!("user3 email = {}, username = {}, active = {}, sign_in_count = {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };


    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };


    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("rect1 is {:?}", rect1); // this does not work because Rectangle does not implement the Debug trait
    // to fix this, we can add #[derive(Debug)] above the struct definition

    let rect4: Rectangle = Rectangle::square(3);
    println!("rect4 is {:?}", rect4);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand
        username,
        active: true,
        sign_in_count: 1,
    }
}