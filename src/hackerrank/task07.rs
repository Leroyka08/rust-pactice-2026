use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = a.iter().fold(1, |acc, &x| lcm(acc, x));
    let gcd_b = b.iter().fold(b[0], |acc, &x| gcd(acc, x));

    (1..=gcd_b)
        .filter(|x| x % lcm_a == 0 && gcd_b % x == 0)
        .count() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_line = stdin_iterator.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let m: usize = first_iter.next().unwrap().parse().unwrap();

    let a_line = stdin_iterator.next().unwrap().unwrap();
    let a: Vec<i32> = a_line
        .split_whitespace()
        .take(n)
        .map(|x| x.parse().unwrap())
        .collect();

    let b_line = stdin_iterator.next().unwrap().unwrap();
    let b: Vec<i32> = b_line
        .split_whitespace()
        .take(m)
        .map(|x| x.parse().unwrap())
        .collect();

    let result = getTotalX(&a, &b);

    writeln!(&mut fptr, "{}", result).ok();
}