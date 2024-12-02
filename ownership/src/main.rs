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

    main2();
    main3();
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

fn main2() {
    println!("main2 - references and borrowing:");
    let s1 = String::from("hello");

    let len = calculate_length2(&s1);

    println!("The length of '{s1}' is {len}.");

    // let s = String::from("hello"); // won't compile

    let mut s = String::from("hello");
    // change(&s); // won't compile
    change(&mut s);

    println!("{s}");

    /*
    // won't compile
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2); */

    /*
    // won't compile
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3); */

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

//won't compile
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main3() {
    // let reference_to_nothing = dangle();
    let _no_dangle = no_dangle();
}

// won't compile
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
