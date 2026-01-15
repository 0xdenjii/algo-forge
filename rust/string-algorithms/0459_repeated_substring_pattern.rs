// LeetCode 459 - Repeated Substring Pattern
// Decide whether s can be built by repeating one of its proper substrings several times.
// Approach: with the KMP prefix function, let p = n - pi[n-1] (smallest period).
// s is a repetition iff pi[n-1] > 0 and n % p == 0.
// Time: O(n), Space: O(n)

pub fn repeated_substring_pattern(s: &str) -> bool {
    let b = s.as_bytes();
    let n = b.len();
    if n < 2 {
        return false;
    }
    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut k = pi[i - 1];
        while k > 0 && b[i] != b[k] {
            k = pi[k - 1];
        }
        if b[i] == b[k] {
            k += 1;
        }
        pi[i] = k;
    }
    let border = pi[n - 1];
    border > 0 && n % (n - border) == 0
}

fn main() {
    assert!(repeated_substring_pattern("abab"));
    assert!(!repeated_substring_pattern("aba"));
    assert!(repeated_substring_pattern("abcabcabcabc"));
    assert!(!repeated_substring_pattern("abac"));
    println!("ok");
}
