#[allow(unused)]
use itertools::Itertools;
use proconio::input;
#[allow(unused)]
use proconio::marker::*;
#[allow(unused)]
use std::collections::*;

fn main() {
    input! {
        h: u8,
        w: u8,
    }
    println!("{}", h.saturating_sub(w));
}
