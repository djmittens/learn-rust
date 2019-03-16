// Aparantly you only need to put this stuff in the main file (extern crates that is)
extern crate primal;
extern crate csv;
extern crate rand;
// Why is this necessary
#[macro_use]
extern crate serde_derive;

mod day2;
mod day3;
mod guessing_game;
mod functions;

fn main() {
    if false {
        println!("Hello, world!");
    }
    if false {
        // Day 2 example
        println!("{:?}", day2::num_divisors(2610, &day2::sieve()));
    }
    if false {
        day3::write_csv();
    }
    if false {
        guessing_game::play_game();
    }

    if !false {
        functions::main();
    }
}
