// LeetCode 2407 - Longest Increasing Subsequence II
// Problem: Return the length of the longest strictly increasing subsequence
//   in which adjacent elements differ by at most k.
// Approach: dp[v] = best length of a valid subsequence ending in value v. For
//   each x: dp[x] = 1 + max(dp[x-k ..= x-1]). An iterative (bottom-up) max
//   segment tree indexed by value answers the range max and the point update.
// Complexity: O(n log M) time, O(M) space, where M = max(nums).

struct MaxTree {
    size: usize,
    t: Vec<u32>,
}

impl MaxTree {
    fn new(n: usize) -> Self {
        MaxTree { size: n, t: vec![0; 2 * n] }
    }

    fn set_max(&mut self, mut i: usize, v: u32) {
        i += self.size;
        self.t[i] = self.t[i].max(v);
        while i > 1 {
            i >>= 1;
            self.t[i] = self.t[2 * i].max(self.t[2 * i + 1]);
        }
    }

    /// Max over the half-open range [l, r).
    fn query(&self, l: usize, r: usize) -> u32 {
        let (mut l, mut r) = (l + self.size, r + self.size);
        let mut best = 0;
        while l < r {
            if l & 1 == 1 {
                best = best.max(self.t[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                best = best.max(self.t[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        best
    }
}

fn length_of_lis(nums: Vec<usize>, k: usize) -> u32 {
    let max = nums.iter().copied().max().unwrap_or(0);
    let mut tree = MaxTree::new(max + 1);
    let mut answer = 0;
    for &x in &nums {
        let best = tree.query(x.saturating_sub(k), x) + 1;
        tree.set_max(x, best);
        answer = answer.max(best);
    }
    answer
}

fn main() {
    assert_eq!(length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
    assert_eq!(length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
    assert_eq!(length_of_lis(vec![1, 5], 1), 1);
    println!("2407 ok");
}
