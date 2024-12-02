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

fn _main5() {
    println!("main5:");

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
        Ordering::Equal | Ordering::Greater => {
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
    // main5();
    // main6();
    another_function(2);
    print_labeled_measurement(6, 'f');
    println!("Five is: {}", five());
    println!("Six is: {}", plus_one(five()));

    main7();
    main8();
    main9();
}

fn another_function(x: i32) {
    println!("another_function:");

    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("print_labeled_measurement:");

    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn _main6() {
    println!("main6");

    println!("Please enter a number:");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: usize = number
        .trim()
        .parse()
        .expect("The number entered was not a number");

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn main7() {
    println!("main7");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn main8() {
    println!("main8");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn main9() {
    println!("main9");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
