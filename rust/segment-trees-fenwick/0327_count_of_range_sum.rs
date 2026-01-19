// LeetCode 327 - Count of Range Sum
// Problem: Given nums and bounds [lower, upper], count the pairs (i, j), i <= j,
//   whose subarray sum nums[i..=j] lies in [lower, upper].
// Approach: With prefix sums P, count the pairs a < b where
//   P[b] - upper <= P[a] <= P[b] - lower. Compress all prefix sums and sweep b
//   left to right. For each b, a Fenwick tree counts the prefixes already
//   inserted inside the value window (bounds found by binary search), then
//   P[b] is inserted.
// Complexity: O(n log n) time, O(n) space.

struct Fenwick(Vec<i64>);

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
    fn prefix(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.0[i];
            i &= i - 1;
        }
        s
    }
}

fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
    let mut prefix = Vec::with_capacity(nums.len() + 1);
    let mut acc = 0i64;
    prefix.push(acc);
    for &x in &nums {
        acc += x as i64;
        prefix.push(acc);
    }
    let mut sorted = prefix.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let (lo, hi) = (lower as i64, upper as i64);
    let mut bit = Fenwick::new(sorted.len());
    let mut count = 0;
    for &p in &prefix {
        // stored values within [p - hi, p - lo]
        let right = sorted.partition_point(|&v| v <= p - lo);
        let left = sorted.partition_point(|&v| v < p - hi);
        if right > left {
            count += bit.prefix(right) - bit.prefix(left);
        }
        bit.add(sorted.binary_search(&p).unwrap() + 1);
    }
    count
}

fn main() {
    assert_eq!(count_range_sum(vec![-2, 5, -1], -2, 2), 3);
    assert_eq!(count_range_sum(vec![0], 0, 0), 1);
    assert_eq!(count_range_sum(vec![i32::MAX, i32::MIN, -1, 0], -1, 0), 4);
    println!("0327 ok");
}
