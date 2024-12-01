use std::cmp::Ordering;
use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main1() {
    println!("main1:");

    println!("There are {THREE_HOURS_IN_SECONDS} seconds in three hours!");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn main2() {
    println!("main2:");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main3() {
    println!("main3:");

    // compile
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // won't compile
    /* let mut spaces = "   ";
    spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
    */
}

fn main4() {
    println!("main4:");
    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;

    let x: (i32, f64, u8) = tup;

    let _five_hundred = x.0;

    let six_point_four = x.1;

    let _one = x.2;

    println!("The value of y is: {six_point_four}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
    let a = [3; 5];
    let _first = a[0];
}

fn main5() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    match index.cmp(&a.len()) {
        Ordering::Less => (),
        Ordering::Equal => {
            println!("The index is too large!");
            return;
        }
        Ordering::Greater => {
            println!("The index is too large!");
            return;
        }
    }

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn main() {
    main1();
    main2();
    main3();
    main4();
    main5();
}
