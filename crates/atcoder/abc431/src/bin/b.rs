#[allow(unused)]
use itertools::Itertools;
use proconio::input;
#[allow(unused)]
use proconio::marker::*;
#[allow(unused)]
use std::collections::*;

fn main() {
    input! {
        mut x: i32,
        n: usize,
        mut w: [i32; n],
        q: usize,
        p: [Usize1; q],
    }
    for &p in &p {
        x += w[p];
        w[p] *= -1;
        println!("{}", x);
    }
}
