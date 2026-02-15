// LeetCode 1235 - Maximum Profit in Job Scheduling
// Jobs have start, end and profit. Choose non-overlapping jobs (a job may start
// exactly when another ends) to maximize total profit.
// Approach: sort by end time; dp[i] = best using first i jobs; binary search
// the last job ending <= current start (weighted interval scheduling).
// Time: O(n log n), Space: O(n)

pub fn job_scheduling(start: &[i32], end: &[i32], profit: &[i64]) -> i64 {
    let mut jobs: Vec<(i32, i32, i64)> = (0..start.len())
        .map(|i| (end[i], start[i], profit[i]))
        .collect();
    jobs.sort_unstable();
    let mut dp = vec![0i64; jobs.len() + 1];
    for i in 0..jobs.len() {
        let (_, s, p) = jobs[i];
        let k = jobs[..i].partition_point(|j| j.0 <= s);
        dp[i + 1] = dp[i].max(dp[k] + p);
    }
    dp[jobs.len()]
}

fn main() {
    assert_eq!(job_scheduling(&[1, 2, 3, 3], &[3, 4, 5, 6], &[50, 10, 40, 70]), 120);
    assert_eq!(
        job_scheduling(&[1, 2, 3, 4, 6], &[3, 5, 10, 6, 9], &[20, 20, 100, 70, 60]),
        150
    );
    println!("ok");
}
