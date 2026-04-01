// Problem 1334: Find the City With the Smallest Number of Neighbors at a Threshold Distance
// n cities, weighted undirected edges, threshold t. Return the city with the fewest other
// cities reachable within distance t; ties go to the largest index.
// Approach: Floyd-Warshall all-pairs shortest paths (k-i-j loop order), then count.
// Complexity: O(n^3) time, O(n^2) space.
pub fn find_the_city(n: usize, edges: &[[usize; 3]], t: usize) -> usize {
    const INF: usize = usize::MAX / 4;
    let mut d = vec![vec![INF; n]; n];
    for (i, row) in d.iter_mut().enumerate() {
        row[i] = 0;
    }
    for &[u, v, w] in edges {
        d[u][v] = d[u][v].min(w);
        d[v][u] = d[v][u].min(w);
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let c = d[i][k] + d[k][j];
                if c < d[i][j] {
                    d[i][j] = c;
                }
            }
        }
    }
    // min_by_key returns the first minimum, so iterate in reverse to prefer larger indices.
    (0..n).rev().min_by_key(|&i| d[i].iter().filter(|&&x| x <= t).count()).unwrap()
}

fn main() {
    assert_eq!(find_the_city(4, &[[0, 1, 3], [1, 2, 1], [1, 3, 4], [2, 3, 1]], 4), 3);
    assert_eq!(
        find_the_city(5, &[[0, 1, 2], [0, 4, 8], [1, 2, 3], [1, 4, 2], [2, 3, 1], [3, 4, 1]], 2),
        0
    );
    println!("ok");
}
