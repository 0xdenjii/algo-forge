// LeetCode 1505 - Minimum Possible Integer After at Most K Adjacent Swaps On Digits
// Problem: Given a digit string num and an integer k, perform at most k swaps
//   of adjacent digits to get the smallest possible string. Leading zeros are
//   allowed.
// Approach: Greedy, one output position at a time: pick the smallest digit d
//   whose nearest remaining occurrence can be moved to the front within the
//   remaining budget. Its cost is the number of not-yet-used digits before it,
//   which is p minus the used indices before p, counted with a Fenwick tree.
// Complexity: O(10 * n log n) time, O(n) space.

use std::collections::VecDeque;

struct Fenwick(Vec<i32>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize) {
        while i < self.0.len() {
            self.0[i] += 1;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.0[i];
            i &= i - 1;
        }
        s
    }
}

fn min_integer(num: &str, mut k: i64) -> String {
    let bytes = num.as_bytes();
    let n = bytes.len();
    let mut queues: Vec<VecDeque<usize>> = vec![VecDeque::new(); 10];
    for (i, &b) in bytes.iter().enumerate() {
        queues[(b - b'0') as usize].push_back(i);
    }
    let mut used = Fenwick::new(n);
    let mut out = String::with_capacity(n);
    for _ in 0..n {
        for d in 0..10 {
            if let Some(&p) = queues[d].front() {
                // Fenwick index p+1 marks original index p; prefix(p) = used indices < p.
                let cost = (p as i32 - used.prefix(p)) as i64;
                if cost <= k {
                    k -= cost;
                    used.add(p + 1);
                    queues[d].pop_front();
                    out.push((b'0' + d as u8) as char);
                    break;
                }
            }
        }
    }
    out
}

fn main() {
    assert_eq!(min_integer("4321", 4), "1342");
    assert_eq!(min_integer("100", 1), "010");
    assert_eq!(min_integer("36789", 1000), "36789");
    assert_eq!(min_integer("22", 22), "22");
    assert_eq!(min_integer("9438957234785635408", 23), "0345989723478563548");
    println!("1505 ok");
}
