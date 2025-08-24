// LeetCode 853 - Car Fleet
// Cars at given positions/speeds drive toward `target`; a faster car that catches
// a slower one joins its fleet. Count fleets arriving at the target.
// Approach: sort by position descending; keep a stack of fleet arrival times.
// A car arriving no later than the fleet ahead merges into it.
// Time O(n log n), Space O(n).
pub fn car_fleet(target: i32, position: &[i32], speed: &[i32]) -> usize {
    let mut cars: Vec<(i32, f64)> = position.iter().zip(speed)
        .map(|(&p, &s)| (p, (target - p) as f64 / s as f64))
        .collect();
    cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));
    let mut fleets: Vec<f64> = Vec::new();
    for (_, t) in cars {
        if fleets.last().map_or(true, |&lead| t > lead) { fleets.push(t); }
    }
    fleets.len()
}

fn main() {
    assert_eq!(car_fleet(12, &[10, 8, 0, 5, 3], &[2, 4, 1, 1, 3]), 3);
    assert_eq!(car_fleet(10, &[3], &[3]), 1);
    assert_eq!(car_fleet(100, &[0, 2, 4], &[4, 2, 1]), 1);
    println!("ok");
}
