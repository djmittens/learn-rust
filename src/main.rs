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
mod vars;

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

    let mut koo = {
        let moo = 5;
        String::from("hello {}")
    };

    koo.push_str(", world!!");

    println!("{}", koo);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sumeusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0,0,0);
    println!("oh noes, we derived something {:?}", user2);

    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 40};
    let rect3 = Rectangle{width: 60, height: 45};

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect1));


    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6;

    // route(four);

    crate::sound::instrument::clarinet();

    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];
    v.push(5);

    let third = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("WHuuuuut"),
        None => println!("There is no god")
    }

}

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            // super::super::main();
        }
    }
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor{r: i32, g: i32, b: i32},
}

fn route(ip_type: IpAddrKind) {}

pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

enum IpAddrKind {
    V4(String),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

struct Color(i32, i32, i32);
struct Paint(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

#[derive(Debug)]
pub struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}