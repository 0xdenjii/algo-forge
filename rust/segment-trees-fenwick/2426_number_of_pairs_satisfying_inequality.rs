// LeetCode 2426 - Number of Pairs Satisfying Inequality
// Problem: Count the pairs i < j with
//   nums1[i] - nums1[j] <= nums2[i] - nums2[j] + diff.
// Approach: Rearranged with a[t] = nums1[t] - nums2[t], the condition is
//   a[i] <= a[j] + diff. Sweep j, count earlier a[i] <= a[j] + diff with a
//   Fenwick tree over the compressed a values (cut point by binary search),
//   then insert a[j].
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

fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
    let a: Vec<i64> = nums1.iter().zip(&nums2).map(|(&x, &y)| x as i64 - y as i64).collect();
    let mut sorted = a.clone();
    sorted.sort_unstable();
    sorted.dedup();
    let mut bit = Fenwick::new(sorted.len());
    let mut total = 0;
    for &v in &a {
        total += bit.prefix(sorted.partition_point(|&s| s <= v + diff as i64));
        bit.add(sorted.binary_search(&v).unwrap() + 1);
    }
    total
}

fn main() {
    assert_eq!(number_of_pairs(vec![3, 2, 5], vec![2, 2, 1], 1), 3);
    assert_eq!(number_of_pairs(vec![3, -1], vec![-2, 2], -1), 0);
    println!("2426 ok");
}
