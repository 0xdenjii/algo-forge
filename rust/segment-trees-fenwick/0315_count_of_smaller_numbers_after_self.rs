// LeetCode 315 - Count of Smaller Numbers After Self
// Problem: For each nums[i], return how many elements to its right are
//   strictly smaller than it.
// Approach: Coordinate-compress the values and sweep from right to left. A
//   Fenwick tree counts the values seen so far; the answer for i is the count
//   of ranks strictly below rank(nums[i]). Then insert nums[i].
// Complexity: O(n log n) time, O(n) space.

struct Fenwick(Vec<i32>);

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick(vec![0; n + 1])
    }
    fn add(&mut self, mut i: usize, d: i32) {
        while i < self.0.len() {
            self.0[i] += d;
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

fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let mut bit = Fenwick::new(sorted.len());
    let mut res = vec![0; nums.len()];
    for (i, &x) in nums.iter().enumerate().rev() {
        let r = sorted.binary_search(&x).unwrap(); // 0-based rank
        res[i] = bit.prefix(r); // Fenwick indices 1..=r hold ranks 0..r
        bit.add(r + 1, 1);
    }
    res
}

fn main() {
    assert_eq!(count_smaller(vec![5, 2, 6, 1]), vec![2, 1, 1, 0]);
    assert_eq!(count_smaller(vec![-1]), vec![0]);
    assert_eq!(count_smaller(vec![-1, -1]), vec![0, 0]);
    println!("0315 ok");
}
