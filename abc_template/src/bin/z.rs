fn main() {
    
}

// const MOD17: usize = 1000000007;
/*
const MOD93: usize = 998244353;
fn mod_add(a: usize, b: usize) -> usize { let ret1 = a % MOD93; let ret2 = b % MOD93; (ret1 + ret2) % MOD93 }
fn mod_sub(a: usize, b: usize) -> usize { let ret1 = a % MOD93; let ret2 = b % MOD93; (ret1 + MOD93 - ret2) % MOD93 }
fn mod_mul(a: usize, b: usize) -> usize { let ret1 = a % MOD93; let ret2 = b % MOD93; (ret1 * ret2) % MOD93 }
*/
// const INV2: usize = 499122177; // MOD93の剰余世界では、1/2の代わりにこれを掛ける
// const INF: usize = 1 << 60;
// const D: [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)]; // 上左下右
// fn us(x: i32) -> usize { x as usize }
// fn iint(x: usize) -> i32 { x as i32 }

#[allow(unused)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
}
#[allow(unused)]
enum Direction {
    UP = 0,
    LEFT = 1,
    DOWN = 2,
    RIGHT = 3,
}
#[allow(unused)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct LoopCount {
    ans: usize, // 答え
    step: usize, // 残りステップ
    pos: usize, // 現在位置
    cycle_len: usize, // サイクル数
    cycle_add: usize, // サイクルによる増加
}
#[allow(unused)]
use proconio::{input, input_interactive, marker::Chars, marker::Usize1};

#[allow(unused)]
use std::{
    mem::swap,
    cmp::min, cmp::max,
    cmp::Reverse,
    collections::HashSet, collections::BTreeSet,
    collections::HashMap, collections::BTreeMap,
    collections::BinaryHeap,
    collections::VecDeque,
    iter::FromIterator,
};
#[allow(unused)]
use itertools::Itertools;
#[allow(unused)]
use num::{integer::gcd, Signed};
#[allow(unused)]
use superslice::Ext;
#[allow(unused)]
use rand::Rng;

// デバッグ用
// 連想配列のデバッグ
#[allow(unused)]
fn debug_print_mp(mp: &HashMap<usize, usize>) {
    for (&k, &v) in mp { println!("key:{} val:{}", k, v); }
}
// 2次元グリッドのデバッグ
#[allow(unused)]
fn debug_print_grid(h: usize, w: usize, grid: &Vec<Vec<char>>) {
    for i in 0..h { for j in 0..w { print!("{}", grid[i][j]); } println!(); }
}
// 最短距離のデバッグ
#[allow(unused)]
fn debug_print_dist(h: usize, w: usize, dist: &Vec<Vec<usize>>) {
    let limit = 255;
    for i in 0..h { for j in 0..w { print!("{:03} ", min(dist[i][j], limit)); } println!(); }
}

// 昇順ソート済みのアサートチェック
#[allow(unused)]
fn assert_sorted_asc<T: Ord>(a: &[T]) {
    debug_assert!(a.windows(2).all(|w| w[0] <= w[1]));
}
// 昇順ソート済みのアサートチェック
#[allow(unused)]
fn assert_sorted_desc<T: Ord>(a: &[T]) {
    debug_assert!(a.windows(2).all(|w| w[0] >= w[1]));
}

// 提出時には出ない
#[macro_export]
#[allow(unused)]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($arg)*)
    };
}

// コール回数の確認
/*
static mut CALLS: usize = 0;
unsafe { CALLS += 1; }
debug!("{}", unsafe {CALLS});
debug_assert!( unsafe {CALLS} < 100_000_000);
*/
