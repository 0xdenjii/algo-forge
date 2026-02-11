// LeetCode 410 - Split Array Largest Sum
// Split nums into k non-empty contiguous subarrays minimizing the largest sum.
// Approach: partition DP dp[j][i] = best for first i elements in j parts,
// using prefix sums. (Binary search on the answer is an O(n log S) alternative.)
// Time: O(k*n^2), Space: O(n)

pub fn split_array(nums: &[i64], k: usize) -> i64 {
    let n = nums.len();
    let mut pre = vec![0i64; n + 1];
    for i in 0..n {
        pre[i + 1] = pre[i] + nums[i];
    }
    let mut dp: Vec<i64> = pre.clone(); // one part
    dp[0] = 0;
    for parts in 2..=k {
        let mut nd = vec![i64::MAX; n + 1];
        for i in parts..=n {
            for p in parts - 1..i {
                let cand = dp[p].max(pre[i] - pre[p]);
                nd[i] = nd[i].min(cand);
            }
        }
        dp = nd;
    }
    dp[n]
}

fn main() {
    assert_eq!(split_array(&[7, 2, 5, 10, 8], 2), 18);
    assert_eq!(split_array(&[1, 2, 3, 4, 5], 2), 9);
    assert_eq!(split_array(&[1, 4, 4], 3), 4);
    println!("ok");
}
