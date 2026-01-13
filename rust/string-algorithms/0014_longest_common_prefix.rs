// LeetCode 14 - Longest Common Prefix
// Return the longest common prefix among an array of strings ("" if none).
// Approach: take the first string and shrink the candidate prefix against every other string.
// Time: O(total chars), Space: O(1) extra

pub fn longest_common_prefix(strs: &[&str]) -> String {
    let Some(first) = strs.first() else { return String::new() };
    let mut len = first.len();
    for s in &strs[1..] {
        len = first
            .bytes()
            .zip(s.bytes())
            .take(len)
            .take_while(|(a, b)| a == b)
            .count();
    }
    // back off to a char boundary for non-ASCII input
    while !first.is_char_boundary(len) {
        len -= 1;
    }
    first[..len].to_string()
}

fn main() {
    assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
    assert_eq!(longest_common_prefix(&["dog", "racecar", "car"]), "");
    assert_eq!(longest_common_prefix(&[]), "");
    println!("ok");
}
