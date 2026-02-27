// LeetCode 1334 - Find the City With the Smallest Number of Neighbors at a Threshold Distance
// Given an undirected weighted graph, return the city with the fewest other cities
// reachable within the threshold; ties go to the largest index.
// Approach: Floyd-Warshall all-pairs shortest paths, then count per row.
// Time: O(V^3), Space: O(V^2)
pub fn find_the_city(n: usize, edges: &[[i32; 3]], threshold: i32) -> usize {
    const INF: i64 = i64::MAX / 4;
    let mut d = vec![vec![INF; n]; n];
    for (i, row) in d.iter_mut().enumerate() {
        row[i] = 0;
    }
    for &[u, v, w] in edges {
        let (u, v) = (u as usize, v as usize);
        d[u][v] = d[u][v].min(w as i64);
        d[v][u] = d[v][u].min(w as i64);
    }
    for k in 0..n {
        for i in 0..n {
            if d[i][k] == INF {
                continue;
            }
            for j in 0..n {
                let c = d[i][k] + d[k][j];
                if c < d[i][j] {
                    d[i][j] = c;
                }
            }
        }
    }
    let mut best = (usize::MAX, 0);
    for i in 0..n {
        let cnt = (0..n).filter(|&j| j != i && d[i][j] <= threshold as i64).count();
        if cnt <= best.0 {
            best = (cnt, i);
        }
    }
    best.1
}

fn main() {
    let e = [[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]];
    assert_eq!(find_the_city(4, &e, 4), 3);
    println!("ok");
}
