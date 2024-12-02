fn main() {
    let s = String::from("hello");
    println!("{s}");

    let mut s = String::from("hello");

    s.push_str(", world!");
    println!("{s}");

    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!"); // This will not compile
    println!("{s2}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let s = String::from("hello");

    takes_ownership(s);

    // let y = s.clone(); // won't compile

    let x = 5;

    makes_copy(x);
    println!("{x}");

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope

    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
