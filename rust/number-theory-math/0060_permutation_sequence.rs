// LeetCode 60 - Permutation Sequence
// Return the k-th (1-indexed) permutation of [1..=n] in lexicographic order, 1 <= n <= 9.
// Approach: factorial number system: the leading digit index is (k-1) / (n-1)!, then recurse.
// Time: O(n^2), Space: O(n)
fn get_permutation(n: usize, k: usize) -> String {
    let mut digits: Vec<u8> = (1..=n as u8).collect();
    let mut fact = vec![1usize; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i;
    }
    let (mut k, mut s) = (k - 1, String::with_capacity(n));
    for i in (0..n).rev() {
        let idx = k / fact[i];
        k %= fact[i];
        s.push(char::from(b'0' + digits.remove(idx)));
    }
    s
}

fn main() {
    assert_eq!(get_permutation(3, 3), "213");
    assert_eq!(get_permutation(4, 9), "2314");
    assert_eq!(get_permutation(3, 1), "123");
    println!("ok");
}
