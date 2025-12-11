// LeetCode 43 - Multiply Strings
// Multiply two non-negative integers given as decimal strings and return the product as a string,
// without converting the inputs to big integers.
// Approach: grade-school multiplication; digit pair (i, j) contributes to positions i+j and i+j+1.
// Time: O(m*n), Space: O(m+n)
fn multiply(a: &str, b: &str) -> String {
    let x: Vec<u32> = a.bytes().map(|c| u32::from(c - b'0')).collect();
    let y: Vec<u32> = b.bytes().map(|c| u32::from(c - b'0')).collect();
    let mut r = vec![0u32; x.len() + y.len()];
    for i in (0..x.len()).rev() {
        for j in (0..y.len()).rev() {
            let s = x[i] * y[j] + r[i + j + 1];
            r[i + j + 1] = s % 10;
            r[i + j] += s / 10;
        }
    }
    let s: String = r
        .iter()
        .skip_while(|&&d| d == 0)
        .map(|&d| char::from(b'0' + d as u8))
        .collect();
    if s.is_empty() { "0".to_string() } else { s }
}

fn main() {
    assert_eq!(multiply("123", "456"), "56088");
    assert_eq!(multiply("0", "52"), "0");
    assert_eq!(multiply("99", "99"), "9801");
    println!("ok");
}
