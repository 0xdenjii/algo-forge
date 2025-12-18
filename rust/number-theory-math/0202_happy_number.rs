// LeetCode 202 - Happy Number
// Repeatedly replace n by the sum of the squares of its digits; n is happy if this reaches 1,
// otherwise it loops forever in a cycle not containing 1.
// Approach: Floyd tortoise-and-hare cycle detection on the sequence.
// Time: O(log n) per step (sequence quickly drops below 243), Space: O(1)
fn next(mut n: u32) -> u32 {
    let mut s = 0;
    while n > 0 {
        let d = n % 10;
        s += d * d;
        n /= 10;
    }
    s
}

fn is_happy(n: u32) -> bool {
    let (mut slow, mut fast) = (n, next(n));
    while fast != 1 && slow != fast {
        slow = next(slow);
        fast = next(next(fast));
    }
    fast == 1
}

fn main() {
    assert!(is_happy(19));
    assert!(!is_happy(2));
    assert!(is_happy(1));
    println!("ok");
}
