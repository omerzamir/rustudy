struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    main1();
    main2();
}

fn main1() {
    println!("main1:");
    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    println!("username: {}", user1.username);
    println!("email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("email: {}", user1.email);

    let user1 = User {
        email: String::from("anotherr@example.com"),
        ..user1
    };
    println!("email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("email: {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main2() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
