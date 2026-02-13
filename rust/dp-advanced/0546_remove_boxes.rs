// LeetCode 546 - Remove Boxes
// Removing k contiguous boxes of the same color earns k*k points. Return the
// maximum points obtainable by removing all boxes.
// Approach: 3D interval DP dp[l][r][k] = best for boxes[l..=r] with k extra
// boxes equal to boxes[l] attached on the left (memoized recursion).
// Time: O(n^4), Space: O(n^3)

pub fn remove_boxes(boxes: &[i32]) -> i32 {
    let n = boxes.len();
    let mut memo = vec![0i32; n * n * n];
    solve(boxes, 0, n as isize - 1, 0, n, &mut memo)
}

fn solve(b: &[i32], mut l: usize, r: isize, mut k: usize, n: usize, memo: &mut [i32]) -> i32 {
    if r < l as isize {
        return 0;
    }
    let r = r as usize;
    let (l0, k0) = (l, k);
    let key = (l0 * n + r) * n + k0;
    if memo[key] != 0 {
        return memo[key];
    }
    while l + 1 <= r && b[l + 1] == b[l] {
        l += 1;
        k += 1;
    }
    let mut best = ((k + 1) * (k + 1)) as i32 + solve(b, l + 1, r as isize, 0, n, memo);
    for m in l + 1..=r {
        if b[m] == b[l] {
            best = best.max(
                solve(b, l + 1, m as isize - 1, 0, n, memo) + solve(b, m, r as isize, k + 1, n, memo),
            );
        }
    }
    memo[key] = best;
    best
}

fn main() {
    assert_eq!(remove_boxes(&[1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
    assert_eq!(remove_boxes(&[1, 1, 1]), 9);
    assert_eq!(remove_boxes(&[1]), 1);
    println!("ok");
}
