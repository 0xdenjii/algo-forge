// LeetCode 171 - Excel Sheet Column Number
// Convert an Excel column title to its column number ("A" -> 1, "AB" -> 28).
// Approach: Horner evaluation in bijective base 26 (A = 1 ... Z = 26).
// Time: O(len), Space: O(1)
fn title_to_number(s: &str) -> u64 {
    s.bytes().fold(0, |acc, c| acc * 26 + u64::from(c - b'A' + 1))
}

fn main() {
    assert_eq!(title_to_number("A"), 1);
    assert_eq!(title_to_number("AB"), 28);
    assert_eq!(title_to_number("ZY"), 701);
    println!("ok");
}
