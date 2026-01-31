// Codeforces 339D - Xenia and Bit Operations
// Problem: A sequence has 2^n elements. Fold it level by level: OR adjacent
//   pairs, then XOR adjacent pairs of the result, alternating until one value
//   v remains. After each point assignment a[p] = b, print v.
// Approach: A perfect binary segment tree whose combine depends on the level:
//   nodes at odd height above the leaves use OR, even heights use XOR. A
//   point update recomputes the O(n) ancestors of the leaf.
// Complexity: build O(2^n), update O(n); space O(2^n).

struct Xenia {
    n: u32,
    size: usize,
    t: Vec<u32>,
}

impl Xenia {
    fn new(n: u32, a: &[u32]) -> Self {
        let size = 1usize << n;
        assert_eq!(a.len(), size);
        let mut t = vec![0; 2 * size];
        t[size..].copy_from_slice(a);
        let mut x = Xenia { n, size, t };
        for i in (1..size).rev() {
            x.pull(i);
        }
        x
    }

    /// Height above the leaves: leaves are 0, the root is n.
    fn height(&self, i: usize) -> u32 {
        self.n - (usize::BITS - 1 - i.leading_zeros())
    }

    fn pull(&mut self, i: usize) {
        let (a, b) = (self.t[2 * i], self.t[2 * i + 1]);
        self.t[i] = if self.height(i) % 2 == 1 { a | b } else { a ^ b };
    }

    fn set(&mut self, pos: usize, v: u32) -> u32 {
        let mut i = pos + self.size;
        self.t[i] = v;
        while i > 1 {
            i >>= 1;
            self.pull(i);
        }
        self.t[1]
    }
}

fn main() {
    // Official sample: n = 2, a = [1, 6, 3, 5]; queries (1-based) -> 1 3 3 3
    let mut x = Xenia::new(2, &[1, 6, 3, 5]);
    let queries = [(1, 4), (3, 4), (1, 2), (1, 2)];
    let answers: Vec<u32> = queries.iter().map(|&(p, b)| x.set(p - 1, b)).collect();
    assert_eq!(answers, vec![1, 3, 3, 3]);
    println!("CF 339D ok");
}
