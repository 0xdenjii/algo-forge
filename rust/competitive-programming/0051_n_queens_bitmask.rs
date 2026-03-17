// Problem 51: N-Queens
// Place n queens on an n x n board so that no two attack each other; return all boards.
// Approach: Row-by-row backtracking with bitmasks for occupied columns and both diagonals;
// the diagonal masks shift by one per row.
// Complexity: O(n!) time worst case, O(n) recursion depth (plus output).
fn go(n: usize, cols: u32, d1: u32, d2: u32, pos: &mut Vec<usize>, out: &mut Vec<Vec<String>>) {
    if pos.len() == n {
        out.push(
            pos.iter()
                .map(|&c| (0..n).map(|j| if j == c { 'Q' } else { '.' }).collect())
                .collect(),
        );
        return;
    }
    let mut free = !(cols | d1 | d2) & ((1u32 << n) - 1);
    while free != 0 {
        let bit = free & free.wrapping_neg();
        free ^= bit;
        pos.push(bit.trailing_zeros() as usize);
        go(n, cols | bit, (d1 | bit) << 1, (d2 | bit) >> 1, pos, out);
        pos.pop();
    }
}

pub fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    assert!(n < 32);
    let mut out = Vec::new();
    go(n, 0, 0, 0, &mut Vec::with_capacity(n), &mut out);
    out
}

fn main() {
    assert_eq!(solve_n_queens(1), vec![vec!["Q".to_string()]]);
    assert_eq!(solve_n_queens(4).len(), 2);
    assert_eq!(solve_n_queens(8).len(), 92);
    for row in &solve_n_queens(4)[0] {
        println!("{row}");
    }
}
