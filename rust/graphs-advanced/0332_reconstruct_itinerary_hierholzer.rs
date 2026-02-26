// LeetCode 332 - Reconstruct Itinerary
// Given airline tickets [from, to], use every ticket exactly once starting at "JFK",
// returning the lexicographically smallest valid itinerary.
// Approach: Hierholzer's Eulerian path. Neighbor lists are reverse-sorted so pop()
// yields the smallest; iterative stack, then reverse the post-order.
// Time: O(E log E), Space: O(E)
use std::collections::HashMap;

pub fn find_itinerary(tickets: &[[&str; 2]]) -> Vec<String> {
    let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
    for &[a, b] in tickets {
        g.entry(a).or_default().push(b);
    }
    for v in g.values_mut() {
        v.sort_unstable_by(|a, b| b.cmp(a));
    }
    let mut stack = vec!["JFK"];
    let mut route = Vec::new();
    while let Some(&top) = stack.last() {
        match g.get_mut(top).and_then(|v| v.pop()) {
            Some(next) => stack.push(next),
            None => route.push(stack.pop().unwrap().to_string()),
        }
    }
    route.reverse();
    route
}

fn main() {
    let t = [["JFK", "SFO"], ["JFK", "ATL"], ["SFO", "ATL"], ["ATL", "JFK"], ["ATL", "SFO"]];
    assert_eq!(find_itinerary(&t), ["JFK", "ATL", "JFK", "SFO", "ATL", "SFO"]);
    let t2 = [["JFK", "KUL"], ["JFK", "NRT"], ["NRT", "JFK"]];
    assert_eq!(find_itinerary(&t2), ["JFK", "NRT", "JFK", "KUL"]);
    println!("ok");
}
