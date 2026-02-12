// LeetCode 464 - Can I Win
// Two players alternately pick distinct integers from 1..=max_choosable and add
// them to a running total; whoever first reaches >= desired_total wins.
// Return whether the first player can force a win.
// Approach: bitmask DP over the set of used numbers (memoized minimax).
// Time: O(2^m * m), Space: O(2^m)

pub fn can_i_win(max_choosable: usize, desired_total: i32) -> bool {
    let m = max_choosable;
    if desired_total <= 0 {
        return true;
    }
    if (m * (m + 1) / 2) < desired_total as usize {
        return false;
    }
    let mut memo = vec![0u8; 1 << m]; // 0 unknown, 1 win, 2 lose
    win(0, desired_total, m, &mut memo)
}

fn win(mask: usize, remaining: i32, m: usize, memo: &mut [u8]) -> bool {
    if memo[mask] != 0 {
        return memo[mask] == 1;
    }
    let res = (0..m).any(|i| {
        mask & (1 << i) == 0
            && ((i as i32 + 1) >= remaining
                || !win(mask | (1 << i), remaining - (i as i32 + 1), m, memo))
    });
    memo[mask] = if res { 1 } else { 2 };
    res
}

fn main() {
    assert!(!can_i_win(10, 11));
    assert!(can_i_win(10, 0));
    assert!(can_i_win(10, 1));
    println!("ok");
}
