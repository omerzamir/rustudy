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

fn main() {
    main1();
    main2();
    main3();
}
