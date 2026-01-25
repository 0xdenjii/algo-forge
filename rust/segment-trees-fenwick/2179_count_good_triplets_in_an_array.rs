// LeetCode 2179 - Count Good Triplets in an Array
// Problem: nums1 and nums2 are permutations of 0..n. Count the triplets of
//   values (x, y, z) that appear in this order in both arrays.
// Approach: Map nums1 through the position of each value in nums2:
//   a[i] = pos2[nums1[i]]. The task becomes counting increasing triplets in a.
//   For each middle j: left = #(i < j, a[i] < a[j]) from a Fenwick tree;
//   right = #(greater values overall) - #(greater values to the left)
//         = (n - 1 - a[j]) - (j - left).
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

fn good_triplets(nums1: Vec<usize>, nums2: Vec<usize>) -> i64 {
    let n = nums1.len();
    let mut pos2 = vec![0usize; n];
    for (i, &v) in nums2.iter().enumerate() {
        pos2[v] = i;
    }
    let mut bit = Fenwick::new(n);
    let mut total = 0;
    for (j, &v) in nums1.iter().enumerate() {
        let a = pos2[v];
        let left = bit.prefix(a); // values 0..a stored at Fenwick indices 1..=a
        let right = (n - 1 - a) as i64 - (j as i64 - left);
        total += left * right;
        bit.add(a + 1);
    }
    total
}

fn main() {
    assert_eq!(good_triplets(vec![2, 0, 1, 3], vec![0, 1, 2, 3]), 1);
    assert_eq!(good_triplets(vec![4, 0, 1, 3, 2], vec![4, 1, 0, 2, 3]), 4);
    println!("2179 ok");
}
