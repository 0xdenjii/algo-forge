// LeetCode 43 - Multiply Strings
// Multiply two non-negative integers given as decimal strings, without big-int libraries.
// Approach: schoolbook multiplication into a digit buffer; digit i*j lands at positions i+j, i+j+1.
// Time: O(m*n), Space: O(m+n)

pub fn multiply(a: &str, b: &str) -> String {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut res = vec![0u32; a.len() + b.len()];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            let sum = (a[i] - b'0') as u32 * (b[j] - b'0') as u32 + res[i + j + 1];
            res[i + j + 1] = sum % 10;
            res[i + j] += sum / 10;
        }
    }
    let s: String = res
        .iter()
        .skip_while(|&&d| d == 0)
        .map(|&d| char::from(b'0' + d as u8))
        .collect();
    if s.is_empty() { "0".into() } else { s }
}

fn main() {
    assert_eq!(multiply("2", "3"), "6");
    assert_eq!(multiply("123", "456"), "56088");
    assert_eq!(multiply("0", "9999"), "0");
    assert_eq!(multiply("99999999999", "99999999999"), "9999999999800000000001");
    println!("ok");
}
