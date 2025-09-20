// LeetCode 744 - Find Smallest Letter Greater Than Target
// Given a sorted list of letters and a target, return the smallest letter
// strictly greater than target, wrapping around to letters[0] if none exists.
// Approach: upper_bound via partition_point, modulo length for wrap-around.
// Time: O(log n), Space: O(1)

fn next_greatest_letter(letters: &[char], target: char) -> char {
    let i = letters.partition_point(|&c| c <= target);
    letters[i % letters.len()]
}

fn main() {
    assert_eq!(next_greatest_letter(&['c', 'f', 'j'], 'a'), 'c');
    assert_eq!(next_greatest_letter(&['c', 'f', 'j'], 'c'), 'f');
    assert_eq!(next_greatest_letter(&['x', 'x', 'y', 'y'], 'z'), 'x');
    println!("ok");
}
