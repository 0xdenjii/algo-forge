// LeetCode 815 - Bus Routes
// routes[i] lists the stops bus i cycles through. Return the minimum number of buses
// needed to travel from source to target, or -1.
// Approach: BFS where each level boards a bus: map stop -> buses, expand every stop of
// an unvisited bus. Each bus and each stop is processed once.
// Time: O(sum of route lengths), Space: O(same)
use std::collections::{HashMap, HashSet, VecDeque};

pub fn num_buses_to_destination(routes: &[Vec<i32>], source: i32, target: i32) -> i32 {
    if source == target {
        return 0;
    }
    let mut by_stop: HashMap<i32, Vec<usize>> = HashMap::new();
    for (bus, r) in routes.iter().enumerate() {
        for &s in r {
            by_stop.entry(s).or_default().push(bus);
        }
    }
    let mut bus_seen = vec![false; routes.len()];
    let mut stop_seen = HashSet::from([source]);
    let mut q = VecDeque::from([(source, 0)]);
    while let Some((stop, d)) = q.pop_front() {
        for &bus in by_stop.get(&stop).into_iter().flatten() {
            if std::mem::replace(&mut bus_seen[bus], true) {
                continue;
            }
            for &nx in &routes[bus] {
                if nx == target {
                    return d + 1;
                }
                if stop_seen.insert(nx) {
                    q.push_back((nx, d + 1));
                }
            }
        }
    }
    -1
}

fn main() {
    assert_eq!(num_buses_to_destination(&[vec![1, 2, 7], vec![3, 6, 7]], 1, 6), 2);
    let r = vec![vec![7, 12], vec![4, 5, 15], vec![6], vec![15, 19], vec![9, 12, 13]];
    assert_eq!(num_buses_to_destination(&r, 15, 12), -1);
    println!("ok");
}
