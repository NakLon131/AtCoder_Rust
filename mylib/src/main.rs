use cargo_snippet::snippet;
use std::collections::{BTreeMap, HashMap};
#[snippet]
pub fn prime_list(x: usize) -> Vec<usize> {
    let mut bools = vec![true; x+1];
    bools[0] = false; bools[1] = false;

    let mut i = 2;
    while i*i <= x {
        if bools[i] { 
            let mut now = 2*i;
            while now <= x {
                bools[now] = false;
                now += i;
            }
        }
        i += 1;
    }

    let mut ret = Vec::new();
    for i in 0..=x { if bools[i] { ret.push(i)}; }
    ret
}

// 素因数分解 連想配列(順序不同)を返す。
pub fn prime_factor(x: usize) -> HashMap<usize, usize> {
    let mut mp = HashMap::new();
    let mut now = x;
    let mut i = 2;
    while i * i <= x {
        while now % i == 0 {
            *mp.entry(i).or_insert(0) += 1;
            now /= i;
        }
        i += 1;
    }
    if now != 1 {
        *mp.entry(now).or_insert(0) += 1;
    }
    mp
}


#[cfg(test)]
mod tests {
    // use super::*;

    use crate::{prime_factor, prime_list};

    // 素数列挙した時の個数
    #[test]
    fn prime_cnt(){
        let primes = prime_list(2_000_000);
        println!("{}", primes.len());
        // assert_eq!(26, primes.len());
    }

    // 約数の個数を列挙
    #[test]
    fn factor_list() {
        for i in 2..=2_000_000 {
            let factor = prime_factor(i);
            let mut factor_cnt = 1;
            
            // let mut factor_map = vec![(0, 0); 0]; // 素因数と個数
            for (_k, v) in factor {
                // factor_map.push((k, v));
                factor_cnt *= v+1;
            }
            // factor_map.sort();

            // 約数の個数を出力
            println!("{} : factor_cnt = {}", i, factor_cnt);
        }
    }

}

fn main() {
    // use crate::prime_factor;

    // 素数列挙した時の個数
    // fn prime_cnt(){
    //     let primes = prime_list(2_000_000);
    //     println!("prime_cnt:{}", primes.len());
    //     for item in primes {
    //         println!("prime_number:{}", item);
    //     }
    //     // assert_eq!(26, primes.len());
    // }
    // prime_cnt();

    // 約数の個数
    // 2*10^6の全列挙は遅いので注意
    // fn factor_list() {
    //     let mut cnt_mp : BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    //     for i in 2..=2_000_000 {
    //         let factor = prime_factor(i);
    //         let mut factor_cnt = 1;
    //         for (_k, v) in factor {
    //             factor_cnt *= v+1;
    //         }
            
    //         // 約数の個数を出力
    //         // println!("{} : factor_cnt = {}", i, factor_cnt);

    //         // 約数の個数毎にmapに入れる
    //         cnt_mp.entry(factor_cnt).or_insert_with(Vec::new).push(i);           
    //     }

    //     for (k, vec) in cnt_mp {
    //         print!("factor_cnt[{}]:", k);
    //         print!("{}", vec.len());
    //         // if k == 3 {
    //         //     for &item in &vec {
    //         //         print!("{} ", item);
    //         //     }
    //         // }
    //         println!();
    //     }
    // }
    // factor_list();
}