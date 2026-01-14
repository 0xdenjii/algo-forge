// LeetCode 187 - Repeated DNA Sequences
// Return all 10-letter sequences that occur more than once in a DNA string (ACGT).
// Approach: encode each nucleotide in 2 bits; rolling 20-bit hash over a window of 10.
// Time: O(n), Space: O(n)

use std::collections::HashSet;

pub fn find_repeated_dna_sequences(s: &str) -> Vec<String> {
    const L: usize = 10;
    let b = s.as_bytes();
    if b.len() <= L {
        return vec![];
    }
    let code = |c: u8| match c {
        b'A' => 0u32,
        b'C' => 1,
        b'G' => 2,
        _ => 3,
    };
    let mask = (1u32 << (2 * L)) - 1;
    let (mut seen, mut out) = (HashSet::new(), HashSet::new());
    let mut h = 0u32;
    for i in 0..b.len() {
        h = ((h << 2) | code(b[i])) & mask;
        if i + 1 >= L && !seen.insert(h) {
            out.insert(&s[i + 1 - L..=i]);
        }
    }
    out.into_iter().map(String::from).collect()
}

fn main() {
    let mut r = find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
    r.sort();
    assert_eq!(r, vec!["AAAAACCCCC", "CCCCCAAAAA"]);
    assert_eq!(find_repeated_dna_sequences("AAAAAAAAAAAAA"), vec!["AAAAAAAAAA"]);
    println!("ok");
}
