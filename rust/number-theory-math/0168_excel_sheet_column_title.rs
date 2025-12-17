// LeetCode 168 - Excel Sheet Column Title
// Convert a positive integer to its Excel column title (1 -> "A", 26 -> "Z", 28 -> "AB").
// Approach: bijective base-26 numeration: decrement before each modulo so digits are 1..=26.
// Time: O(log n), Space: O(log n)
fn convert_to_title(mut n: u32) -> String {
    let mut s = Vec::new();
    while n > 0 {
        n -= 1;
        s.push(b'A' + (n % 26) as u8);
        n /= 26;
    }
    s.reverse();
    String::from_utf8(s).expect("ASCII letters")
}

fn main() {
    assert_eq!(convert_to_title(1), "A");
    assert_eq!(convert_to_title(28), "AB");
    assert_eq!(convert_to_title(701), "ZY");
    println!("ok");
}
