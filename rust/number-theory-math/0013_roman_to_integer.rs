// LeetCode 13 - Roman to Integer
// Convert a valid Roman numeral (I, V, X, L, C, D, M with subtractive pairs like IV, CM) to an integer.
// Approach: scan left to right; a symbol smaller than its successor is subtracted, else added.
// Time: O(n), Space: O(1)
fn value(c: u8) -> i32 {
    match c {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        b'M' => 1000,
        _ => panic!("invalid roman symbol"),
    }
}

fn roman_to_int(s: &str) -> i32 {
    let b = s.as_bytes();
    (0..b.len())
        .map(|i| {
            let v = value(b[i]);
            if i + 1 < b.len() && v < value(b[i + 1]) { -v } else { v }
        })
        .sum()
}

fn main() {
    assert_eq!(roman_to_int("III"), 3);
    assert_eq!(roman_to_int("LVIII"), 58);
    assert_eq!(roman_to_int("MCMXCIV"), 1994);
    println!("ok");
}
