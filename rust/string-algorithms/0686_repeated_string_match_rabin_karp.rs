// LeetCode 686 - Repeated String Match
// Return the minimum number of times a must be repeated so that b is a substring of it, or -1.
// Approach: repeat a q = ceil(|b|/|a|) times; b can only appear in q or q+1 copies.
// Search with Rabin-Karp (rolling polynomial hash, verified on hash hit).
// Time: O(|a| + |b|) expected, Space: O(|a| + |b|)

const BASE: u64 = 131;
const MOD: u64 = 1_000_000_007;

fn rabin_karp(text: &[u8], pat: &[u8]) -> bool {
    let m = pat.len();
    if m > text.len() {
        return false;
    }
    let hash = |s: &[u8]| s.iter().fold(0u64, |h, &c| (h * BASE + c as u64) % MOD);
    let target = hash(pat);
    let mut pow = 1u64; // BASE^(m-1)
    for _ in 1..m {
        pow = pow * BASE % MOD;
    }
    let mut h = hash(&text[..m]);
    for i in 0..=text.len() - m {
        if h == target && &text[i..i + m] == pat {
            return true;
        }
        if i + m < text.len() {
            h = (h + MOD - text[i] as u64 * pow % MOD) % MOD;
            h = (h * BASE + text[i + m] as u64) % MOD;
        }
    }
    false
}

pub fn repeated_string_match(a: &str, b: &str) -> i32 {
    let q = b.len().div_ceil(a.len()).max(1);
    let text = a.repeat(q + 1);
    let tb = text.as_bytes();
    if rabin_karp(&tb[..q * a.len()], b.as_bytes()) {
        q as i32
    } else if rabin_karp(tb, b.as_bytes()) {
        q as i32 + 1
    } else {
        -1
    }
}

fn main() {
    assert_eq!(repeated_string_match("abcd", "cdabcdab"), 3);
    assert_eq!(repeated_string_match("a", "aa"), 2);
    assert_eq!(repeated_string_match("abc", "wxyz"), -1);
    println!("ok");
}
