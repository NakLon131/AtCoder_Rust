fn main() {
    
}

// const MOD17: usize = 1000000007;
// const MOD93: usize = 998244353;
// const INV2: usize = 499122177; // MOD93の剰余世界では、1/2の代わりにこれを掛ける
// const INF: usize = 1 << 60;
// const D: [Pos; 4] = [Pos{x:!0, y:0}, Pos{x:0, y:!0}, Pos{x:1, y:0}, Pos{x:0, y:1}]; // 上左下右
// const D: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)]; // 上左下右

#[allow(unused)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
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
use num::integer::gcd;
#[allow(unused)]
use superslice::Ext;
#[allow(unused)]
use ac_library::{
    FenwickTree
    // ModInt998244353 as mint,
    // ModInt1000000007 as mint
};

// 連想配列のデバッグ
#[allow(unused)]
fn dbg_print_mp(mp: &HashMap<usize, usize>) {
    for (&k, &v) in mp { println!("key:{} val:{}", k, v); }
}
// グリッドのデバッグ
#[allow(unused)]
fn dbg_print_grid(h: usize, w: usize, grid: &Vec<Vec<char>>) {
    for i in 0..h { for j in 0..w { print!("{}", grid[i][j]); } println!(); }
}
// 最短距離のデバッグ
#[allow(unused)]
fn dbg_print_dist(h: usize, w: usize, dist: &Vec<Vec<usize>>) {
    let limit = 255;
    for i in 0..h { for j in 0..w { print!("{:03} ", min(dist[i][j], limit)); } println!(); }
}
