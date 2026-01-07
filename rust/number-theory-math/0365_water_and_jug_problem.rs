// LeetCode 365 - Water and Jug Problem
// With two jugs of capacities x and y and unlimited water, can you end with exactly target
// liters total in the jugs (fill, empty, pour operations)?
// Approach: Bezout's identity: reachable amounts are multiples of gcd(x, y) not exceeding x + y.
// Time: O(log min(x, y)), Space: O(1)
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn can_measure_water(x: u64, y: u64, target: u64) -> bool {
    if target > x + y {
        return false;
    }
    match gcd(x, y) {
        0 => target == 0,
        g => target % g == 0,
    }
}

fn main() {
    assert!(can_measure_water(3, 5, 4));
    assert!(!can_measure_water(2, 6, 5));
    assert!(can_measure_water(1, 2, 3));
    println!("ok");
}
