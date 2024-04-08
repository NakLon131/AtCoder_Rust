#[allow(unused_macros)]
macro_rules! read {
    ([$t:ty] ; $n:expr) =>
        ((0..$n).map(|_| read!([$t])).collect::<Vec<_>>());
    ($($t:ty),+ ; $n:expr) =>
        ((0..$n).map(|_| read!($($t),+)).collect::<Vec<_>>());
    ([$t:ty]) =>
        (rl().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<$t>>());
    ($t:ty) =>
        (rl().parse::<$t>().unwrap());
    ($($t:ty),*) => {{
        let buf = rl();
        let mut w = buf.split_whitespace();
        ($(w.next().unwrap().parse::<$t>().unwrap()),*)
    }};
}

#[allow(dead_code)]
fn rl() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_owned()
}
#[allow(unused)]
use std::{
	io::stderr, io::stdin, io::BufRead, io::Write,
	mem::swap,
	cmp::min, cmp::max,
	cmp::Reverse,
	collections::HashSet, collections::BTreeSet,
	collections::HashMap, collections::BTreeMap,
	collections::BinaryHeap,
	collections::VecDeque,
};

fn main() {
    let x = read!(usize);
    println!("{}", x*x*x);
}