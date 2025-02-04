#![feature(const_trait_impl, effects)]
use std::fmt::Display;

fn main() {
    pretty_print_int(solve(7));
    pretty_print_int(solve(51));
}

const fn solve(n: u128) -> u128 {
    match n {
        0 => 0,
        _ => 2 * solve(n - 1) + n,
    }
}

fn pretty_print_int<T>(i: T)
where
    T: Display,
{
    let mut s = String::new();
    let i_str = i.to_string();
    let a = i_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    println!("{}", s);
}
