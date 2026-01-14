// LeetCode 151 - Reverse Words in a String
// Reverse the order of words; collapse repeated spaces and trim the ends.
// Approach: split_whitespace, reverse the iterator, join with single spaces.
// Time: O(n), Space: O(n)

pub fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

fn main() {
    assert_eq!(reverse_words("the sky is blue"), "blue is sky the");
    assert_eq!(reverse_words("  hello world  "), "world hello");
    assert_eq!(reverse_words("a good   example"), "example good a");
    println!("ok");
}
