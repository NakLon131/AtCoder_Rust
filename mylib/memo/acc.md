


fn main() {
    

}

#[cfg(test)]
mod tests {
    use crate::{calc_acc, get_range_acc, get_rev_idx};
    use crate::{calc_racc, get_range_racc};
    #[test]
    fn left_acc_test() {
        let vec = vec![1, 3, 5, 7, 9];

        let acc = calc_acc(&vec);
        assert_eq!(0, acc[0]);
        assert_eq!(4, acc[2]);
        assert_eq!(25, acc[5]);

        assert_eq!(12, get_range_acc(3, 4, &acc));
        assert_eq!(0, get_range_acc(0, 0, &acc));
        assert_eq!(1, get_range_acc(1, 1, &acc));
        assert_eq!(25, get_range_acc(1, 5, &acc));
    }
    
    #[test]
    fn right_acc_test() {
        let vec = vec![1, 3, 5, 7, 9];

        let racc = calc_racc(&vec);
        assert_eq!(0, racc[get_rev_idx(5, 0)]);
        assert_eq!(21, racc[get_rev_idx(5, 3)]);
        assert_eq!(25, racc[get_rev_idx(5, 5)]);

        assert_eq!(0, get_range_racc(0, 0, &racc));
        assert_eq!(9, get_range_racc(1, 1, &racc));
        assert_eq!(1, get_range_racc(5, 5, &racc));
        assert_eq!(25, get_range_racc(5, 1, &racc));
        assert_eq!(12, get_range_racc(3, 2, &racc));
        assert_eq!(12, get_range_racc(2, 3, &racc));
    }

}

const SZ: usize = 5;
// 左から累積和 先頭からi個足したときの値
#[allow(unused)]
fn calc_acc(vec: &Vec<usize>) -> Vec<usize> 
{
    let mut acc = vec![0; SZ+1];
    for i in 0..SZ { acc[i+1] = acc[i] + vec[i]; }
    acc
}
#[allow(unused)]
// 左から[from..to]の累積和
fn get_range_acc(mut from: usize, mut to: usize, acc: &Vec<usize>) -> usize
{
    // from,toの大小が逆なら入れ替える
    if from > to { (from, to) = (to, from); }
    // fromが0の場合
    if from == 0 { return acc[to]; }
    // [from..to]
    acc[to] - acc[from-1]
}

// 右から累積和 末尾からi個足したときの値(indexは左からになっているので注意)
#[allow(unused)]
fn calc_racc(vec: &Vec<usize>) -> Vec<usize> 
{
    let mut racc = vec![0; SZ+1];
    for i in (0..SZ).rev() { racc[i] = racc[i+1] + vec[i]; }
    racc
}
// 右から累積和について、右から数えたindexを返す
#[allow(unused)]
fn get_rev_idx(n: usize, pos: usize) -> usize { n - pos }

// 右から[from..to]の累積和(from,toは右からのindexを指定)
#[allow(unused)]
fn get_range_racc(mut rfrom: usize, mut rto: usize, racc: &Vec<usize>) -> usize
{
    // from,toの大小が逆なら入れ替える
    if rfrom > rto { (rfrom, rto) = (rto, rfrom); }
    // fromが0の場合
    let n = racc.len()-1;
    if rfrom == 0 { return racc[get_rev_idx(n, rto)];}
    // [from..to]
    racc[get_rev_idx(n, rto)] - racc[get_rev_idx(n, rfrom-1)]
}