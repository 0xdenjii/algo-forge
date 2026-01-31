// CSES 1143 - Hotel Queries
// Problem: Hotels in a row have free-room counts. Groups arrive in order;
//   each group of size g takes rooms in the first (leftmost) hotel with at
//   least g free rooms. Print that hotel's 1-based index, or 0 if there is
//   none.
// Approach: A max segment tree over the free counts. Descend from the root:
//   go left if the left child's max >= g, otherwise go right. Then subtract g
//   at the leaf and recompute the ancestors.
// Complexity: O(log n) per group, O(n) space.

struct Hotels {
    size: usize,
    t: Vec<i64>,
}

impl Hotels {
    fn new(rooms: &[i64]) -> Self {
        let size = rooms.len().next_power_of_two();
        let mut t = vec![0; 2 * size];
        t[size..size + rooms.len()].copy_from_slice(rooms);
        for i in (1..size).rev() {
            t[i] = t[2 * i].max(t[2 * i + 1]);
        }
        Hotels { size, t }
    }

    /// Assigns the group and returns the 1-based hotel index, or 0.
    fn assign(&mut self, g: i64) -> usize {
        if self.t[1] < g {
            return 0;
        }
        let mut i = 1;
        while i < self.size {
            i = if self.t[2 * i] >= g { 2 * i } else { 2 * i + 1 };
        }
        let hotel = i - self.size;
        self.t[i] -= g;
        while i > 1 {
            i >>= 1;
            self.t[i] = self.t[2 * i].max(self.t[2 * i + 1]);
        }
        hotel + 1
    }
}

fn main() {
    // Official sample
    let mut h = Hotels::new(&[3, 2, 4, 1, 5, 5, 2, 6]);
    let out: Vec<usize> = [4, 4, 7, 1, 1].iter().map(|&g| h.assign(g)).collect();
    assert_eq!(out, vec![3, 5, 0, 1, 1]);
    println!("CSES Hotel Queries ok");
}
