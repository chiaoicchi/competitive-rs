#[allow(unused)]
use itertools::Itertools;
use proconio::input;
#[allow(unused)]
use proconio::marker::*;
#[allow(unused)]
use std::collections::*;

const N: usize = 500 * 500 / 2 + 1;
const INF: i64 = 1 << 40;

fn main() {
    input! {
        n: usize,
        whb: [(usize, i64, i64); n],
    }

    let mut dp = [-INF; N];
    dp[0] = 0;

    for &(w, h, b) in &whb {
        let mut swp = [-INF; N];
        for i in 0..N {
            if dp[i] != -INF {
                swp[i] = swp[i].max(dp[i] + b);
                if i + w < N {
                    swp[i + w] = swp[i + w].max(dp[i] + h);
                }
            }
        }
        dp = swp;
    }

    let tot_w = whb.iter().map(|x| x.0).sum::<usize>();
    let mut ans = !0;
    for i in 0..=tot_w / 2 {
        let j = tot_w - i;
        if i <= j {
            ans = ans.max(dp[i]);
        }
    }

    println!("{}", ans);
}
