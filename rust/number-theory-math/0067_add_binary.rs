// LeetCode 67 - Add Binary
// Given two binary strings a and b, return their sum as a binary string.
// Approach: walk both strings from the right with a carry bit.
// Time: O(max(m, n)), Space: O(max(m, n))
fn add_binary(a: &str, b: &str) -> String {
    let (mut i, mut j) = (a.bytes().rev(), b.bytes().rev());
    let (mut carry, mut out) = (0u8, Vec::new());
    loop {
        let (x, y) = (i.next(), j.next());
        if x.is_none() && y.is_none() && carry == 0 {
            break;
        }
        let s = x.map_or(0, |c| c - b'0') + y.map_or(0, |c| c - b'0') + carry;
        out.push(b'0' + s % 2);
        carry = s / 2;
    }
    if out.is_empty() {
        out.push(b'0');
    }
    out.reverse();
    String::from_utf8(out).expect("binary digits")
}

fn main() {
    assert_eq!(add_binary("11", "1"), "100");
    assert_eq!(add_binary("1010", "1011"), "10101");
    assert_eq!(add_binary("0", "0"), "0");
    println!("ok");
}
