// LeetCode 326 - Power of Three
// Return true if n is a power of three (without loops ideally).
// Approach: 3^19 = 1_162_261_467 is the largest power of 3 fitting in i32; since 3 is prime,
// n > 0 is a power of three iff it divides 3^19.
// Time: O(1), Space: O(1)
fn is_power_of_three(n: i32) -> bool {
    n > 0 && 1_162_261_467 % n == 0
}

fn main() {
    assert!(is_power_of_three(27));
    assert!(is_power_of_three(1));
    assert!(!is_power_of_three(45));
    assert!(!is_power_of_three(0));
    println!("ok");
}
