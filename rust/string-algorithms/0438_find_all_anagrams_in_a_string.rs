// LeetCode 438 - Find All Anagrams in a String
// Given lowercase strings s and p, return all start indices of p's anagrams in s.
// Approach: fixed-size sliding window of letter counts; track how many of the 26 letters match.
// Time: O(n + m), Space: O(1)

pub fn find_anagrams(s: &str, p: &str) -> Vec<usize> {
    let (s, p) = (s.as_bytes(), p.as_bytes());
    let m = p.len();
    if m == 0 || m > s.len() {
        return vec![];
    }
    let mut diff = [0i32; 26]; // window count minus pattern count
    for &c in p {
        diff[(c - b'a') as usize] -= 1;
    }
    let mut zeros = diff.iter().filter(|&&d| d == 0).count();
    let bump =|diff: &mut [i32; 26], zeros: &mut usize, c: u8, delta: i32| {
        let i = (c - b'a') as usize;
        if diff[i] == 0 {
            *zeros -= 1;
        }
        diff[i] += delta;
        if diff[i] == 0 {
            *zeros += 1;
        }
    };
    let mut out = Vec::new();
    for i in 0..s.len() {
        bump(&mut diff, &mut zeros, s[i], 1);
        if i >= m {
            bump(&mut diff, &mut zeros, s[i - m], -1);
        }
        if i + 1 >= m && zeros == 26 {
            out.push(i + 1 - m);
        }
    }
    out
}

fn main() {
    assert_eq!(find_anagrams("cbaebabacd", "abc"), vec![0, 6]);
    assert_eq!(find_anagrams("abab", "ab"), vec![0, 1, 2]);
    println!("ok");
}
