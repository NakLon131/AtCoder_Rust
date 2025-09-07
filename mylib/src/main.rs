use cargo_snippet::snippet;
use rand::Rng;
//use Rand::Rng;

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

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::prime_list;

    #[test]
    fn test1(){
        // 2*10^6 -> 素数は148933個
        let primes = prime_list(2_000_000);
        print!("{}", primes.len());
        // assert_eq!(26, primes.len());
    }
}


fn main() {
    let mut x = rand::thread_rng();
    let i = x.gen_range(0..100);

    println!("{}", i);
}