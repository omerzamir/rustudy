use rand::Rng;
use std::collections::HashMap;
use std::collections::HashMap as ColHashMap;
use std::{self, cmp::Ordering, io};
// use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// use crate::front_of_house::hosting;

// Import and re-export
pub use crate::front_of_house::hosting;

use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);

    let mut map2 = ColHashMap::new();
    map2.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
