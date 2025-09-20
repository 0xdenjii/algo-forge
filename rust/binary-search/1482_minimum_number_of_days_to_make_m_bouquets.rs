// LeetCode 1482 - Minimum Number of Days to Make m Bouquets
// bloom_day[i] is when flower i blooms. A bouquet needs k adjacent bloomed
// flowers. Return the minimum day to make m bouquets, or -1 if impossible.
// Approach: binary search on the day; count bouquets greedily for a given day.
// Time: O(n log maxDay), Space: O(1)

fn bouquets(bloom: &[i32], day: i32, k: i32) -> i64 {
    let (mut made, mut run) = (0i64, 0);
    for &b in bloom {
        if b <= day {
            run += 1;
            if run == k {
                made += 1;
                run = 0;
            }
        } else {
            run = 0;
        }
    }
    made
}

fn min_days(bloom: &[i32], m: i32, k: i32) -> i32 {
    if (m as i64) * (k as i64) > bloom.len() as i64 {
        return -1;
    }
    let (mut lo, mut hi) = (*bloom.iter().min().unwrap(), *bloom.iter().max().unwrap());
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if bouquets(bloom, mid, k) >= m as i64 { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(min_days(&[1, 10, 3, 10, 2], 3, 1), 3);
    assert_eq!(min_days(&[1, 10, 3, 10, 2], 3, 2), -1);
    assert_eq!(min_days(&[7, 7, 7, 7, 12, 7, 7], 2, 3), 12);
    println!("ok");
}
