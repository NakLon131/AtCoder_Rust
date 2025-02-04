// 問題を解くときに気を付けること
// ・入出力例を全て確認する。
// ・制約に気を付ける。(極端に小さい、大きい、オーダーはいくつか？)
// ・ソートされているか？(問題文に書かれていないことがある。)
// ・図に書いてみて考える。
// ・単調増加や単調減少が成り立つか？
// ・最大値や最大値を求める問題か？(二分探索を考える)
// ・問題の見方を変えてみる。(縦や横に見る、偶奇を考える、逆からやる)
fn main() {
    
}

// const MOD17: usize = 1000000007;
// const MOD93: usize = 998244353;
// const INF: usize = 1 << 60;
// const D : [(usize, usize); 4] = [(!0, 0), (0, !0), (1, 0), (0, 1)]; // 上左下右

#[allow(unused)]
use proconio::{input, marker::Chars, marker::Usize1};

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
	FenwickTree,
	// ModInt998244353 as mint,
	// ModInt1000000007 as mint
};

// 構造体を使う例
// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
// struct Block {
//     x: usize,
//     y: usize,
//     idx: usize,
// }