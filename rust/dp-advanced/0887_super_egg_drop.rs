// LeetCode 887 - Super Egg Drop
// With k eggs and a building of n floors, find the minimum number of moves
// that guarantees determining the critical floor f.
// Approach: invert the question: f[m][k] = floors coverable with m moves and
// k eggs = f[m-1][k-1] + f[m-1][k] + 1. Find smallest m with f >= n.
// Time: O(k * log n) moves-ish (O(k*m)), Space: O(k)

pub fn super_egg_drop(k: usize, n: u64) -> u64 {
    let mut f = vec![0u64; k + 1];
    let mut moves = 0;
    while f[k] < n {
        moves += 1;
        for e in (1..=k).rev() {
            f[e] = f[e] + f[e - 1] + 1;
        }
    }
    moves
}

fn main() {
    assert_eq!(super_egg_drop(1, 2), 2);
    assert_eq!(super_egg_drop(2, 6), 3);
    assert_eq!(super_egg_drop(3, 14), 4);
    assert_eq!(super_egg_drop(2, 100), 14);
    println!("ok");
}
