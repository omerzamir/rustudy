mod garden;
mod omer;

use crate::garden::vegetables::Asparagus;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}

pub use crate::omer::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
