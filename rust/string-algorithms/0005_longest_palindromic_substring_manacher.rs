// LeetCode 5 - Longest Palindromic Substring
// Given a string s, return the longest palindromic substring in s.
// Approach: Manacher's algorithm on the transformed string ^#a#b#...#$ which unifies odd/even
// centers; reuse the mirror radius inside the current rightmost palindrome.
// Time: O(n), Space: O(n)

pub fn longest_palindrome(s: &str) -> String {
    let b = s.as_bytes();
    if b.is_empty() {
        return String::new();
    }
    let mut t = Vec::with_capacity(2 * b.len() + 3);
    t.push(b'^');
    for &c in b {
        t.push(b'#');
        t.push(c);
    }
    t.push(b'#');
    t.push(b'$');
    let n = t.len();
    let mut p = vec![0usize; n];
    let (mut center, mut right) = (0usize, 0usize);
    for i in 1..n - 1 {
        if i < right {
            p[i] = p[2 * center - i].min(right - i);
        }
        while t[i + p[i] + 1] == t[i - p[i] - 1] {
            p[i] += 1;
        }
        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }
    }
    // leftmost center with the maximal radius
    let (mut ci, mut len) = (0, 0);
    for (i, &v) in p.iter().enumerate() {
        if v > len {
            ci = i;
            len = v;
        }
    }
    let start = (ci - len) / 2;
    s[start..start + len].to_string()
}

fn main() {
    assert_eq!(longest_palindrome("babad"), "bab");
    assert_eq!(longest_palindrome("cbbd"), "bb");
    assert_eq!(longest_palindrome("forgeeksskeegfor"), "geeksskeeg");
    println!("ok");
}
