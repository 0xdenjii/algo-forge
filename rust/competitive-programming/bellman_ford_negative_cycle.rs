// Title: Bellman-Ford with Negative Cycle Detection
// Problem: Single-source shortest paths in a directed graph that may contain negative edges;
// report an error if a negative cycle is reachable from the source.
// Approach: Relax all edges up to V-1 times (early exit when stable); a successful relaxation
// in round V proves a reachable negative cycle.
// Complexity: O(VE) time, O(V) space.
#[derive(Debug, PartialEq)]
pub struct NegativeCycle;

pub fn bellman_ford(
    n: usize,
    edges: &[(usize, usize, i64)],
    s: usize,
) -> Result<Vec<Option<i64>>, NegativeCycle> {
    let mut d: Vec<Option<i64>> = vec![None; n];
    d[s] = Some(0);
    for round in 0..n {
        let mut changed = false;
        for &(u, v, w) in edges {
            if let Some(du) = d[u] {
                if d[v].map_or(true, |dv| du + w < dv) {
                    d[v] = Some(du + w);
                    changed = true;
                }
            }
        }
        if !changed {
            return Ok(d);
        }
        if round == n - 1 {
            return Err(NegativeCycle);
        }
    }
    Ok(d)
}

fn main() {
    let e = [(0, 1, 4), (0, 2, 5), (1, 2, -3), (2, 3, 4)];
    assert_eq!(bellman_ford(4, &e, 0), Ok(vec![Some(0), Some(4), Some(1), Some(5)]));
    let neg = [(0, 1, 1), (1, 2, -2), (2, 1, 1)];
    assert_eq!(bellman_ford(3, &neg, 0), Err(NegativeCycle));
    println!("ok");
}
