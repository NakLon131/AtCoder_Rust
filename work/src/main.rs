use std::collections::VecDeque;
use num::Signed;
use proconio::{input, marker::Chars};
type Pos = (i32, i32);
const INF: i32 = 1 << 30;
const D4 : [Pos; 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
fn us(x: i32) -> usize { x as usize }
fn main() {
    input! {
        h: i32, w: i32, // 区画の行と列
        s: [Chars; h], // 区画のマス目
    }
    // 01bfs
    let ans = bfs01(h, w, &s);
    println!("{}", ans);
}

fn bfs01(h: i32, w: i32, s: &Vec<Vec<char>>) -> i32 {
    // スタート地点
    let spos: Pos = (0, 0);
    // 各マスの最短距離
    let mut min_dist = vec![vec![INF; us(w)]; us(h)];
    min_dist[us(spos.0)][us(spos.1)] = 0;
    // 座標のキュー
    let mut dq = VecDeque::new();
    dq.push_back(spos);

    while let Some(cpos) = dq.pop_front() {
        // そのまま移動できる場合
        go_straight(h, w, s, cpos, &mut min_dist, &mut dq);
        // 壁を壊して移動する場合
        go_break(h, w, s, cpos, &mut min_dist, &mut dq);
    }
    min_dist[us(h-1)][us(w-1)]
}

fn go_straight(h: i32, w: i32, s: &Vec<Vec<char>>, 
    cpos: (i32, i32), min_dist: &mut Vec<Vec<i32>>, dq: &mut VecDeque<(i32, i32)>)
{
    for d in D4 {
        let npos = (cpos.0 + d.0, cpos.1 + d.1);
        if (npos.0 < 0 || npos.0 >= h) || (npos.1 < 0 || npos.1 >= w) { continue; }    
        if s[us(npos.0)][us(npos.1)] != '.' { continue; }
        if min_dist[us(npos.0)][us(npos.1)] > min_dist[us(cpos.0)][us(cpos.1)] {
            min_dist[us(npos.0)][us(npos.1)] = min_dist[us(cpos.0)][us(cpos.1)];
            dq.push_front(npos);
        }
    }
}

fn go_break(h: i32, w: i32, _s: &Vec<Vec<char>>, 
    cpos: (i32, i32), min_dist: &mut Vec<Vec<i32>>, dq: &mut VecDeque<(i32, i32)>) 
{
    for i in -2..=2 {
        for j in -2..=2 {
            if i.abs() + j.abs() == 4 { continue; }
            if i == 0 && j == 0 { continue; }
            let npos = (cpos.0 + i, cpos.1 + j);
            if (npos.0 < 0 || npos.0 >= h) || (npos.1 < 0 || npos.1 >= w) { continue; }
            if min_dist[us(npos.0)][us(npos.1)] > min_dist[us(cpos.0)][us(cpos.1)] + 1 {
                min_dist[us(npos.0)][us(npos.1)] = min_dist[us(cpos.0)][us(cpos.1)] + 1;
                dq.push_back(npos);
            }
        }
    }
}