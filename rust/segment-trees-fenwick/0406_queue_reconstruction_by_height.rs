// LeetCode 406 - Queue Reconstruction by Height
// Problem: people[i] = [h, k] describes a person of height h with exactly k
//   people of height >= h standing in front. Reconstruct the queue.
// Approach: Process people by height ascending (ties: k descending). Every
//   slot still empty will be taken by someone at least as tall, so the person
//   goes into the (k+1)-th empty slot. A Fenwick tree over "slot is empty"
//   finds that slot with a binary-lifting descent.
// Complexity: O(n log n) time, O(n) space.

struct Fenwick(Vec<i32>);

impl Fenwick {
    fn with_ones(n: usize) -> Self {
        let mut t = vec![0; n + 1];
        for i in 1..=n {
            t[i] += 1;
            let parent = i + (i & i.wrapping_neg());
            if parent <= n {
                t[parent] += t[i];
            }
        }
        Fenwick(t)
    }
    fn add(&mut self, mut i: usize, d: i32) {
        while i < self.0.len() {
            self.0[i] += d;
            i += i & i.wrapping_neg();
        }
    }
    /// Smallest 1-based index whose prefix sum reaches k (k >= 1).
    fn kth(&self, mut k: i32) -> usize {
        let n = self.0.len() - 1;
        let mut pos = 0;
        let mut step = if n == 0 { 0 } else { 1 << (usize::BITS - 1 - n.leading_zeros()) };
        while step > 0 {
            if pos + step <= n && self.0[pos + step] < k {
                pos += step;
                k -= self.0[pos];
            }
            step >>= 1;
        }
        pos + 1
    }
}

fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = people.len();
    people.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));
    let mut empty = Fenwick::with_ones(n);
    let mut res = vec![Vec::new(); n];
    for p in people {
        let pos = empty.kth(p[1] + 1);
        empty.add(pos, -1);
        res[pos - 1] = p;
    }
    res
}

fn main() {
    let input = vec![vec![7, 0], vec![4, 4], vec![7, 1], vec![5, 0], vec![6, 1], vec![5, 2]];
    let expected = vec![vec![5, 0], vec![7, 0], vec![5, 2], vec![6, 1], vec![4, 4], vec![7, 1]];
    assert_eq!(reconstruct_queue(input), expected);
    let input = vec![vec![6, 0], vec![5, 0], vec![4, 0], vec![3, 2], vec![2, 2], vec![1, 4]];
    let expected = vec![vec![4, 0], vec![5, 0], vec![2, 2], vec![3, 2], vec![1, 4], vec![6, 0]];
    assert_eq!(reconstruct_queue(input), expected);
    println!("0406 ok");
}
