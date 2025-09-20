// LeetCode 1011 - Capacity To Ship Packages Within D Days
// Packages with given weights must be shipped in order, one or more per day,
// total weight per day <= capacity. Return the least capacity that ships all
// packages within `days` days.
// Approach: binary search capacity in [max(w), sum(w)], greedy day count check.
// Time: O(n log sum), Space: O(1)

fn days_needed(weights: &[i32], cap: i32) -> i32 {
    let (mut days, mut load) = (1, 0);
    for &w in weights {
        if load + w > cap {
            days += 1;
            load = 0;
        }
        load += w;
    }
    days
}

fn ship_within_days(weights: &[i32], days: i32) -> i32 {
    let mut lo = *weights.iter().max().unwrap();
    let mut hi: i32 = weights.iter().sum();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if days_needed(weights, mid) <= days { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(ship_within_days(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
    assert_eq!(ship_within_days(&[3, 2, 2, 4, 1, 4], 3), 6);
    assert_eq!(ship_within_days(&[1, 2, 3, 1, 1], 4), 3);
    println!("ok");
}
