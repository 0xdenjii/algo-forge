// LeetCode 307 - Range Sum Query - Mutable
// Problem: Given an integer array, support two operations:
//   update(index, val) sets nums[index] = val, and
//   sum_range(left, right) returns the sum of nums[left..=right].
// Approach: Fenwick (binary indexed) tree over the values, built in O(n).
//   An update adds the delta (new - old); a range sum is prefix(r+1) - prefix(l).
// Complexity: build O(n), update/query O(log n); space O(n).

struct NumArray {
    bit: Vec<i64>,
    vals: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut bit = vec![0i64; n + 1];
        for (i, &v) in nums.iter().enumerate() {
            let k = i + 1;
            bit[k] += v as i64;
            let parent = k + (k & k.wrapping_neg());
            if parent <= n {
                bit[parent] += bit[k];
            }
        }
        NumArray { bit, vals: nums }
    }

    fn add(&mut self, mut i: usize, delta: i64) {
        while i < self.bit.len() {
            self.bit[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i &= i - 1;
        }
        s
    }

    fn update(&mut self, index: usize, val: i32) {
        let delta = val as i64 - self.vals[index] as i64;
        self.vals[index] = val;
        self.add(index + 1, delta);
    }

    fn sum_range(&self, left: usize, right: usize) -> i32 {
        (self.prefix(right + 1) - self.prefix(left)) as i32
    }
}

fn main() {
    let mut na = NumArray::new(vec![1, 3, 5]);
    assert_eq!(na.sum_range(0, 2), 9);
    na.update(1, 2);
    assert_eq!(na.sum_range(0, 2), 8);
    assert_eq!(na.sum_range(1, 1), 2);
    println!("0307 ok");
}
