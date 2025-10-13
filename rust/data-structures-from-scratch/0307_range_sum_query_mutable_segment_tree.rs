// LeetCode 307 - Range Sum Query - Mutable
// Given an array, support update(index, val) and sum_range(left, right) inclusive.
// Approach: iterative bottom-up segment tree of size 2n; leaves at [n, 2n), parents at i = (2i, 2i+1).
// Time: O(log n) per op, O(n) build. Space: O(n).
pub struct NumArray {
    n: usize,
    tree: Vec<i64>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut tree = vec![0i64; 2 * n];
        for (i, &x) in nums.iter().enumerate() {
            tree[n + i] = x as i64;
        }
        for i in (1..n).rev() {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }
        Self { n, tree }
    }

    pub fn update(&mut self, index: i32, val: i32) {
        let mut i = index as usize + self.n;
        self.tree[i] = val as i64;
        while i > 1 {
            i /= 2;
            self.tree[i] = self.tree[2 * i] + self.tree[2 * i + 1];
        }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        // half-open [l, r)
        let (mut l, mut r) = (left as usize + self.n, right as usize + self.n + 1);
        let mut s = 0i64;
        while l < r {
            if l & 1 == 1 {
                s += self.tree[l];
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                s += self.tree[r];
            }
            l /= 2;
            r /= 2;
        }
        s as i32
    }
}

fn main() {
    let mut a = NumArray::new(vec![1, 3, 5]);
    assert_eq!(a.sum_range(0, 2), 9);
    a.update(1, 2);
    assert_eq!(a.sum_range(0, 2), 8);
    assert_eq!(a.sum_range(1, 1), 2);

    // brute-force cross-check
    let mut v: Vec<i32> = (0..37).map(|i| (i * 7919 % 23) - 11).collect();
    let mut st = NumArray::new(v.clone());
    for step in 0..200usize {
        let i = step * 13 % v.len();
        v[i] = (step as i32 * 31) % 17 - 8;
        st.update(i as i32, v[i]);
        let (l, r) = ((step * 5) % v.len(), (step * 11) % v.len());
        let (l, r) = (l.min(r), l.max(r));
        assert_eq!(st.sum_range(l as i32, r as i32), v[l..=r].iter().sum::<i32>());
    }
    println!("segment tree ok");
}
