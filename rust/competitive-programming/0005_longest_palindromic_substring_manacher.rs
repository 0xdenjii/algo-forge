// Problem 5: Longest Palindromic Substring
// Return the longest palindromic substring of s (the leftmost one on ties).
// Approach: Manacher's algorithm on the transformed string "#a#b#...#" so every palindrome
// has odd length; reuse mirrored radii inside the rightmost known palindrome.
// Complexity: O(n) time and space.
pub fn longest_palindrome(s: &str) -> &str {
    let b = s.as_bytes();
    let t: Vec<u8> = std::iter::once(b'#').chain(b.iter().flat_map(|&c| [c, b'#'])).collect();
    let n = t.len();
    let mut p = vec![0usize; n];
    let (mut c, mut r) = (0usize, 0usize);
    for i in 0..n {
        if i < r {
            p[i] = (r - i).min(p[2 * c - i]);
        }
        while i > p[i] && i + p[i] + 1 < n && t[i - p[i] - 1] == t[i + p[i] + 1] {
            p[i] += 1;
        }
        if i + p[i] > r {
            c = i;
            r = i + p[i];
        }
    }
    // p[i] equals the palindrome length in the original string.
    let (mut best_i, mut best_len) = (0, 0);
    for (i, &len) in p.iter().enumerate() {
        if len > best_len {
            best_i = i;
            best_len = len;
        }
    }
    let start = (best_i - best_len) / 2;
    &s[start..start + best_len]
}

fn main() {
    assert_eq!(longest_palindrome("babad"), "bab");
    assert_eq!(longest_palindrome("cbbd"), "bb");
    assert_eq!(longest_palindrome("forgeeksskeegfor"), "geeksskeeg");
    assert_eq!(longest_palindrome(""), "");
    println!("ok");
}
