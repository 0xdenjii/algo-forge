// LeetCode 9 - Palindrome Number
// Determine whether an integer reads the same backward as forward, without converting to a string.
// Approach: reverse only the second half of the digits and compare with the first half.
// Time: O(log10 x), Space: O(1)
fn is_palindrome(mut x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut rev = 0;
    while x > rev {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    x == rev || x == rev / 10
}

fn main() {
    assert!(is_palindrome(121));
    assert!(!is_palindrome(-121));
    assert!(!is_palindrome(10));
    assert!(is_palindrome(0));
    println!("ok");
}
