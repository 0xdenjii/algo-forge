// Classic - Merge Sort Tree (count of elements less than x in a range)
// Problem: Given a static array, answer online queries (l, r, x): how many
//   elements of a[l..=r] are strictly less than x? (Compare SPOJ KQUERYO.)
// Approach: Each segment tree node stores the sorted list of its range, built
//   bottom-up by merging the children's lists. A query splits [l, r] into
//   O(log n) canonical nodes and binary-searches x in each one.
// Complexity: build O(n log n), query O(log^2 n); space O(n log n).

struct MergeSortTree {
    size: usize,
    nodes: Vec<Vec<i32>>,
}

fn merge(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            out.push(a[i]);
            i += 1;
        } else {
            out.push(b[j]);
            j += 1;
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

impl MergeSortTree {
    fn new(a: &[i32]) -> Self {
        let size = a.len().next_power_of_two();
        let mut nodes = vec![Vec::new(); 2 * size];
        for (i, &v) in a.iter().enumerate() {
            nodes[size + i] = vec![v];
        }
        for i in (1..size).rev() {
            nodes[i] = merge(&nodes[2 * i], &nodes[2 * i + 1]);
        }
        MergeSortTree { size, nodes }
    }

    fn count_less(&self, l: usize, r: usize, x: i32) -> usize {
        let (mut l, mut r) = (l + self.size, r + self.size + 1);
        let mut total = 0;
        while l < r {
            if l & 1 == 1 {
                total += self.nodes[l].partition_point(|&v| v < x);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                total += self.nodes[r].partition_point(|&v| v < x);
            }
            l >>= 1;
            r >>= 1;
        }
        total
    }
}

fn main() {
    let a = [5, 1, 4, 2, 8, 7, 3, 6, 2, 9];
    let t = MergeSortTree::new(&a);
    assert_eq!(t.count_less(0, 4, 4), 2); // 1, 2
    assert_eq!(t.count_less(2, 8, 5), 4); // 4, 2, 3, 2
    for l in 0..a.len() {
        for r in l..a.len() {
            for x in 0..11 {
                let naive = a[l..=r].iter().filter(|&&v| v < x).count();
                assert_eq!(t.count_less(l, r, x), naive);
            }
        }
    }
    println!("merge sort tree ok");
}
