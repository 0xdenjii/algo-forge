// LeetCode 3 - Longest Substring Without Repeating Characters
// Given a string s, find the length of the longest substring without repeated characters.
// Approach: sliding window; remember the last index of every byte and jump the left edge past it.
// Time: O(n), Space: O(1) (256-entry table)

pub fn length_of_longest_substring(s: &str) -> usize {
    let mut last = [usize::MAX; 256];
    let (mut left, mut best) = (0usize, 0usize);
    for (i, &b) in s.as_bytes().iter().enumerate() {
        let prev = last[b as usize];
        if prev != usize::MAX && prev >= left {
            left = prev + 1;
        }
        last[b as usize] = i;
        best = best.max(i + 1 - left);
    }
    best
}

fn main() {
    assert_eq!(length_of_longest_substring("abcabcbb"), 3);
    assert_eq!(length_of_longest_substring("bbbbb"), 1);
    assert_eq!(length_of_longest_substring("pwwkew"), 3);
    assert_eq!(length_of_longest_substring(""), 0);
    println!("ok");
}
