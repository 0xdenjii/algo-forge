// LeetCode 673 - Number of Longest Increasing Subsequence
// Problem: Return how many strictly increasing subsequences have the
//   maximum possible length.
// Approach: Compress the values. A Fenwick tree over ranks stores, for each
//   prefix, the pair (best length, number of ways). A merge keeps the longer
//   length, or adds the counts when the lengths tie. For x at rank r, query
//   ranks < r, extend by one, and merge the result back in at r.
// Complexity: O(n log n) time, O(n) space.

use std::cmp::Ordering;

#[derive(Clone, Copy, Default, Debug)]
struct Best {
    len: u32,
    cnt: u64,
}

fn merge(a: Best, b: Best) -> Best {
    match a.len.cmp(&b.len) {
        Ordering::Greater => a,
        Ordering::Less => b,
        Ordering::Equal => Best { len: a.len, cnt: a.cnt + b.cnt },
    }
}

struct Fenwick(Vec<Best>);

impl Fenwick {
    fn update(&mut self, mut i: usize, v: Best) {
        while i < self.0.len() {
            self.0[i] = merge(self.0[i], v);
            i += i & i.wrapping_neg();
        }
    }
    fn query(&self, mut i: usize) -> Best {
        let mut acc = Best::default();
        while i > 0 {
            acc = merge(acc, self.0[i]);
            i &= i - 1;
        }
        acc
    }
}

fn find_number_of_lis(nums: Vec<i32>) -> u64 {
    let mut sorted = nums.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let mut bit = Fenwick(vec![Best::default(); sorted.len() + 1]);
    for &x in &nums {
        let r = sorted.binary_search(&x).unwrap() + 1;
        let prev = bit.query(r - 1);
        let cur = if prev.len == 0 {
            Best { len: 1, cnt: 1 }
        } else {
            Best { len: prev.len + 1, cnt: prev.cnt }
        };
        bit.update(r, cur);
    }
    bit.query(sorted.len()).cnt
}

fn main() {
    assert_eq!(find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
    assert_eq!(find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
    assert_eq!(find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2]), 3);
    println!("0673 ok");
}
