// LeetCode 858 - Mirror Reflection
// A square room of side p has receptors 0, 1, 2 at corners. A laser from the SW corner first hits
// the east wall at distance q. Return the receptor it meets first.
// Approach: unfold reflections; the beam hits a corner after lcm(p,q); parity of the multiples decides.
// Time: O(log min(p,q)) | Space: O(1)
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn mirror_reflection(p: u64, q: u64) -> u8 {
    let g = gcd(p, q);
    let (m, n) = (q / g, p / g); // m = vertical room count, n = horizontal reflections count
    match (m % 2, n % 2) {
        (0, _) => 0,
        (1, 1) => 1,
        _ => 2,
    }
}

fn main() {
    assert_eq!(mirror_reflection(2, 1), 2);
    assert_eq!(mirror_reflection(3, 1), 1);
    assert_eq!(mirror_reflection(4, 2), 2);
    println!("ok");
}
