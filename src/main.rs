// Aparantly you only need to put this stuff in the main file (extern crates that is)
extern crate primal;

mod day2;

fn main() {
    println!("Hello, world!");
    // Day 2 example
    println!("{:?}", day2::num_divisors(2610, &day2::sieve()));
}
