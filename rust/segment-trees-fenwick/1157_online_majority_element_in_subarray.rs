// LeetCode 1157 - Online Majority Element In Subarray
// Problem: Preprocess arr, then answer query(left, right, threshold): return
//   the element that occurs at least `threshold` times in arr[left..=right]
//   (guaranteed 2*threshold > length), or -1 if there is none.
// Approach: A segment tree whose nodes hold Boyer-Moore voting state
//   (candidate, count). That merge is associative, so a range query gives the
//   only possible majority candidate. Its real frequency comes from binary
//   searches on that value's sorted position list.
// Complexity: build O(n), query O(log n); space O(n).

use std::collections::HashMap;

#[derive(Clone, Copy, Default)]
struct Vote {
    cand: i32,
    cnt: i32,
}

fn merge(a: Vote, b: Vote) -> Vote {
    if a.cand == b.cand {
        Vote { cand: a.cand, cnt: a.cnt + b.cnt }
    } else if a.cnt >= b.cnt {
        Vote { cand: a.cand, cnt: a.cnt - b.cnt }
    } else {
        Vote { cand: b.cand, cnt: b.cnt - a.cnt }
    }
}

struct MajorityChecker {
    size: usize,
    tree: Vec<Vote>,
    positions: HashMap<i32, Vec<usize>>,
}

impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let size = arr.len().next_power_of_two();
        let mut tree = vec![Vote::default(); 2 * size];
        let mut positions: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in arr.iter().enumerate() {
            tree[size + i] = Vote { cand: v, cnt: 1 };
            positions.entry(v).or_default().push(i);
        }
        for i in (1..size).rev() {
            tree[i] = merge(tree[2 * i], tree[2 * i + 1]);
        }
        MajorityChecker { size, tree, positions }
    }

    fn query(&self, left: usize, right: usize, threshold: usize) -> i32 {
        // Iterative bottom-up fold; the merge is commutative enough for voting.
        let (mut l, mut r) = (left + self.size, right + self.size + 1);
        let mut acc = Vote::default();
        while l < r {
            if l & 1 == 1 {
                acc = merge(acc, self.tree[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                acc = merge(acc, self.tree[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        let Some(pos) = self.positions.get(&acc.cand) else { return -1 };
        let freq = pos.partition_point(|&p| p <= right) - pos.partition_point(|&p| p < left);
        if freq >= threshold { acc.cand } else { -1 }
    }
}

fn main() {
    let mc = MajorityChecker::new(vec![1, 1, 2, 2, 1, 1]);
    assert_eq!(mc.query(0, 5, 4), 1);
    assert_eq!(mc.query(0, 3, 3), -1);
    assert_eq!(mc.query(2, 3, 2), 2);
    println!("1157 ok");
}
