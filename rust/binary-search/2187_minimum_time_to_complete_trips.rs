// LeetCode 2187 - Minimum Time to Complete Trips
// Bus i needs time[i] per trip, buses run independently and in parallel.
// Return the minimum time for all buses together to complete at least
// total_trips trips.
// Approach: trips(t) = sum(t / time[i]) is monotone; binary search t.
// Time: O(n log(min_time * total_trips)), Space: O(1)

fn minimum_time(time: &[i32], total_trips: i32) -> i64 {
    let need = total_trips as i64;
    let trips = |t: i64| {
        let mut s = 0i64;
        for &x in time {
            s += t / x as i64;
            if s >= need {
                break;
            }
        }
        s
    };
    let (mut lo, mut hi) = (1i64, *time.iter().min().unwrap() as i64 * need);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if trips(mid) >= need { hi = mid } else { lo = mid + 1 }
    }
    lo
}

fn main() {
    assert_eq!(minimum_time(&[1, 2, 3], 5), 3);
    assert_eq!(minimum_time(&[2], 1), 2);
    assert_eq!(minimum_time(&[10_000_000], 10_000_000), 100_000_000_000_000);
    println!("ok");
}
