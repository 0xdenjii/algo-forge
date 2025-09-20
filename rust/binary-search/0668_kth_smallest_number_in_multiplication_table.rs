// LeetCode 668 - Kth Smallest Number in Multiplication Table
// The m x n multiplication table has entry i*j at (i, j), 1-based. Return the
// kth smallest number in it.
// Approach: binary search value v; count(v) = sum over rows i of min(v / i, n).
// Time: O(m log(m*n)), Space: O(1)

fn find_kth_number(m: i64, n: i64, k: i64) -> i64 {
    let count = |v: i64| (1..=m).map(|i| (v / i).min(n)).sum::<i64>();
    let (mut lo, mut hi) = (1, m * n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if count(mid) >= k { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(find_kth_number(3, 3, 5), 3);
    assert_eq!(find_kth_number(2, 3, 6), 6);
    assert_eq!(find_kth_number(1, 1, 1), 1);
    println!("ok");
}
