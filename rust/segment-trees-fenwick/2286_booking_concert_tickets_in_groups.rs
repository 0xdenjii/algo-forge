// LeetCode 2286 - Booking Concert Tickets in Groups
// Problem: A hall has n rows of m seats. gather(k, max_row) seats k people
//   together in one row <= max_row (the smallest such row, leftmost free
//   seats) and returns [row, seat], or [] if impossible. scatter(k, max_row)
//   seats k people anywhere in rows <= max_row (smallest rows first) and
//   returns whether it succeeded.
// Approach: A segment tree over rows keeps min(used seats) and sum(used
//   seats). gather descends to the first row <= max_row with
//   used <= m - k. scatter checks free capacity with the sum, then fills
//   rows from a pointer to the first non-full row, which only moves forward.
// Complexity: gather O(log n); scatter O(log n) amortized; space O(n).

struct BookMyShow {
    n: usize,
    m: i64,
    min_used: Vec<i64>,
    sum_used: Vec<i64>,
    first_free: usize,
}

impl BookMyShow {
    fn new(n: usize, m: i64) -> Self {
        BookMyShow { n, m, min_used: vec![0; 4 * n], sum_used: vec![0; 4 * n], first_free: 0 }
    }

    fn add(&mut self, node: usize, l: usize, r: usize, idx: usize, v: i64) {
        if l == r {
            self.min_used[node] += v;
            self.sum_used[node] += v;
            return;
        }
        let mid = (l + r) / 2;
        if idx <= mid {
            self.add(2 * node, l, mid, idx, v);
        } else {
            self.add(2 * node + 1, mid + 1, r, idx, v);
        }
        self.min_used[node] = self.min_used[2 * node].min(self.min_used[2 * node + 1]);
        self.sum_used[node] = self.sum_used[2 * node] + self.sum_used[2 * node + 1];
    }

    fn sum(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.sum_used[node];
        }
        let mid = (l + r) / 2;
        self.sum(2 * node, l, mid, ql, qr) + self.sum(2 * node + 1, mid + 1, r, ql, qr)
    }

    /// First row index <= qr whose used count is <= limit.
    fn first_fit(&self, node: usize, l: usize, r: usize, qr: usize, limit: i64) -> Option<usize> {
        if l > qr || self.min_used[node] > limit {
            return None;
        }
        if l == r {
            return Some(l);
        }
        let mid = (l + r) / 2;
        self.first_fit(2 * node, l, mid, qr, limit)
            .or_else(|| self.first_fit(2 * node + 1, mid + 1, r, qr, limit))
    }

    fn gather(&mut self, k: i64, max_row: usize) -> Vec<i64> {
        match self.first_fit(1, 0, self.n - 1, max_row, self.m - k) {
            Some(row) => {
                let used = self.sum(1, 0, self.n - 1, row, row);
                self.add(1, 0, self.n - 1, row, k);
                vec![row as i64, used]
            }
            None => vec![],
        }
    }

    fn scatter(&mut self, mut k: i64, max_row: usize) -> bool {
        let free = (max_row as i64 + 1) * self.m - self.sum(1, 0, self.n - 1, 0, max_row);
        if free < k {
            return false;
        }
        while k > 0 {
            let row = self.first_free;
            let used = self.sum(1, 0, self.n - 1, row, row);
            let take = (self.m - used).min(k);
            if take > 0 {
                self.add(1, 0, self.n - 1, row, take);
                k -= take;
            }
            if used + take == self.m {
                self.first_free += 1;
            }
        }
        true
    }
}

fn main() {
    let mut show = BookMyShow::new(2, 5);
    assert_eq!(show.gather(4, 0), vec![0, 0]);
    assert_eq!(show.gather(2, 0), Vec::<i64>::new());
    assert!(show.scatter(5, 1));
    assert!(!show.scatter(5, 1));
    println!("2286 ok");
}
