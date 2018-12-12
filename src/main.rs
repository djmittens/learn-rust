// Aparantly you only need to put this stuff in the main file (extern crates that is)
extern crate primal;
extern crate csv;
// Why is this necessary
#[macro_use]
extern crate serde_derive;

mod day2;
mod day3;

fn main() {
    println!("Hello, world!");
    // Day 2 example
    println!("{:?}", day2::num_divisors(2610, &day2::sieve()));
    day3::write_csv();
}
