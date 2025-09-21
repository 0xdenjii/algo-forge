// SPOJ AGGRCOW - Aggressive Cows
// Given n stall positions and c cows, place the cows in stalls so that the
// minimum distance between any two cows is as large as possible. Return that
// largest minimum distance.
// Approach: sort stalls; binary search the distance d; greedily place cows
// left to right and check whether c fit (feasibility is monotone in d).
// Time: O(n log n + n log range), Space: O(1) besides sort

fn can_place(stalls: &[i64], cows: usize, d: i64) -> bool {
    let mut placed = 1;
    let mut last = stalls[0];
    for &s in &stalls[1..] {
        if s - last >= d {
            placed += 1;
            last = s;
            if placed == cows {
                return true;
            }
        }
    }
    placed >= cows
}

fn largest_min_distance(mut stalls: Vec<i64>, cows: usize) -> i64 {
    stalls.sort_unstable();
    // invariant: lo feasible, hi infeasible
    let (mut lo, mut hi) = (0i64, stalls[stalls.len() - 1] - stalls[0] + 1);
    while hi - lo > 1 {
        let mid = lo + (hi - lo) / 2;
        if can_place(&stalls, cows, mid) { lo = mid } else { hi = mid }
    }
    lo
}

fn main() {
    assert_eq!(largest_min_distance(vec![1, 2, 8, 4, 9], 3), 3);
    assert_eq!(largest_min_distance(vec![0, 3, 4, 7, 10, 9], 4), 3);
    assert_eq!(largest_min_distance(vec![5, 5], 2), 0);
    println!("ok");
}
