extern crate primal;

use primal::Sieve;

pub fn sieve() -> Sieve {
    let sieve = Sieve::new(10000);
        let suspect = 5273;
        println!("{} is prime: {}", suspect, sieve.is_prime(suspect));
        let not_a_prime = 1024;
        println!("{} is prime: {}", not_a_prime, sieve.is_prime(not_a_prime));

    let n = 1000;
        match sieve.primes_from(0).nth(n -1) {
            Some(number) => println!("{}th prime is {}", n, number),
            None => println!("I don't know anything about {}th prime.", n),
        }
        println!("{:?}", sieve.factor(2610));
    return sieve;
}

pub fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}