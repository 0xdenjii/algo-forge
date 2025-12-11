// LeetCode 7 - Reverse Integer
// Reverse the digits of a 32-bit signed integer; return 0 if the result overflows i32.
// Approach: pop digits with % and /, push with checked arithmetic to detect overflow.
// Time: O(log10 x), Space: O(1)
fn reverse(mut x: i32) -> i32 {
    let mut r: i32 = 0;
    while x != 0 {
        r = match r.checked_mul(10).and_then(|v| v.checked_add(x % 10)) {
            Some(v) => v,
            None => return 0,
        };
        x /= 10;
    }
    r
}

fn main() {
    assert_eq!(reverse(123), 321);
    assert_eq!(reverse(-120), -21);
    assert_eq!(reverse(1_534_236_469), 0);
    println!("ok");
}
