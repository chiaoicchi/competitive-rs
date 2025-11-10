#[allow(unused)]
use itertools::Itertools;
use proconio::input;
#[allow(unused)]
use proconio::marker::*;
#[allow(unused)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        mut h: [u32; n],
        mut b: [u32; m],
    }
    h.sort();
    b.sort();
    if h[..k]
        .iter()
        .zip(b[b.len() - k..].iter())
        .all(|(h, b)| h <= b)
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
