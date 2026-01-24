// LeetCode 493 - Reverse Pairs
// Problem: Count the pairs (i, j) with i < j and nums[i] > 2 * nums[j].
// Approach: Compress the values and sweep j left to right. The number of
//   earlier values strictly greater than 2*nums[j] is
//   inserted - (#inserted <= 2*nums[j]). A Fenwick tree keeps the counts, and
//   a binary search on the sorted values finds the cut point. i64 arithmetic
//   avoids overflow.
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

fn reverse_pairs(nums: Vec<i32>) -> i64 {
    let mut sorted: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
    sorted.sort_unstable();
    sorted.dedup();
    let mut bit = Fenwick::new(sorted.len());
    let mut total = 0;
    for (inserted, &x) in nums.iter().enumerate() {
        let twice = 2 * x as i64;
        let le = sorted.partition_point(|&v| v <= twice);
        total += inserted as i64 - bit.prefix(le);
        bit.add(sorted.binary_search(&(x as i64)).unwrap() + 1);
    }
    total
}

fn main() {
    assert_eq!(reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    assert_eq!(reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    assert_eq!(reverse_pairs(vec![i32::MAX, i32::MAX, i32::MIN]), 2);
    println!("0493 ok");
}
