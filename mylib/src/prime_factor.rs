use std::collections::HashMap;

use cargo_snippet::snippet;

#[snippet]
pub fn prime_factor(x: usize) -> HashMap<usize, usize> {
    let mut mp = HashMap::new();
    
    let mut now = x;
    let mut i = 2;
    while i*i <= x {
        while now % i == 0 {
            *mp.entry(i).or_insert(0) += 1;
            now /= i;
        }
        i += 1;
    }
    if now != 1 {*mp.entry(now).or_insert(0) += 1; }
    mp
}

#[cfg(test)]
mod tests {
    // use super::*;

    use crate::prime_factor;

    #[test]
    fn test1(){
        // 460 = 2*2 * 5 * 23
        let mp = prime_factor(460);
        assert_eq!(3, mp.len());
        assert_eq!(2, *mp.get(&2).unwrap());
        assert_eq!(1, *mp.get(&5).unwrap());
        assert_eq!(1, *mp.get(&23).unwrap());
    }
    #[test]
    fn test2(){
        // 512 = 2*9
        let mp = prime_factor(512);
        assert_eq!(1, mp.len());
        assert_eq!(9, *mp.get(&2).unwrap());
    }
}

fn main() {
    
}