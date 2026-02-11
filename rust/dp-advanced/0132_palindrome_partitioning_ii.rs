// LeetCode 132 - Palindrome Partitioning II
// Return the minimum number of cuts needed to partition s so that every
// part is a palindrome.
// Approach: expand around each center; for each palindrome s[l..=r],
// cuts[r+1] = min(cuts[r+1], cuts[l] + 1) with cuts[0] = -1.
// Time: O(n^2), Space: O(n)

pub fn min_cut(s: &str) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut cuts: Vec<i32> = (0..=n as i32).map(|i| i - 1).collect();
    for c in 0..n {
        for (mut l, mut r) in [(c as isize, c as isize), (c as isize, c as isize + 1)] {
            while l >= 0 && (r as usize) < n && s[l as usize] == s[r as usize] {
                let (lu, ru) = (l as usize, r as usize);
                cuts[ru + 1] = cuts[ru + 1].min(cuts[lu] + 1);
                l -= 1;
                r += 1;
            }
        }
    }
    cuts[n]
}

fn main() {
    assert_eq!(min_cut("aab"), 1);
    assert_eq!(min_cut("a"), 0);
    assert_eq!(min_cut("ab"), 1);
    assert_eq!(min_cut("racecarannakayak"), 2);
    println!("ok");
}
