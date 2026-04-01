// Problem 1109: Corporate Flight Bookings
// n flights; each booking [first, last, seats] adds seats to every flight in the inclusive
// 1-indexed range. Return the total seats reserved per flight.
// Approach: Difference array: d[first-1] += s, d[last] -= s, then prefix sum.
// Complexity: O(n + q) time, O(n) space.
pub fn corp_flight_bookings(bookings: &[[usize; 3]], n: usize) -> Vec<i64> {
    let mut d = vec![0i64; n + 1];
    for &[f, l, s] in bookings {
        d[f - 1] += s as i64;
        d[l] -= s as i64;
    }
    d.truncate(n);
    let mut acc = 0;
    for x in d.iter_mut() {
        acc += *x;
        *x = acc;
    }
    d
}

fn main() {
    assert_eq!(
        corp_flight_bookings(&[[1, 2, 10], [2, 3, 20], [2, 5, 25]], 5),
        vec![10, 55, 45, 25, 25]
    );
    assert_eq!(corp_flight_bookings(&[[1, 2, 10], [2, 2, 15]], 2), vec![10, 25]);
    println!("ok");
}
