// LeetCode 214 - Shortest Palindrome
// Add the fewest characters in front of s to make it a palindrome; return the result.
// Approach: KMP prefix function of s + '#' + reverse(s) yields the longest palindromic prefix;
// prepend the reversed remainder.
// Time: O(n), Space: O(n)

fn prefix_function(p: &[u8]) -> Vec<usize> {
    let mut pi = vec![0; p.len()];
    for i in 1..p.len() {
        let mut k = pi[i - 1];
        while k > 0 && p[i] != p[k] {
            k = pi[k - 1];
        }
        if p[i] == p[k] {
            k += 1;
        }
        pi[i] = k;
    }
    pi
}

pub fn shortest_palindrome(s: &str) -> String {
    let rev: String = s.chars().rev().collect();
    let combined = format!("{s}#{rev}");
    let pal_len = prefix_function(combined.as_bytes()).last().copied().unwrap_or(0);
    let suffix = &s[pal_len..];
    suffix.chars().rev().collect::<String>() + s
}

fn main() {
    assert_eq!(shortest_palindrome("aacecaaa"), "aaacecaaa");
    assert_eq!(shortest_palindrome("abcd"), "dcbabcd");
    assert_eq!(shortest_palindrome(""), "");
    println!("ok");
}
