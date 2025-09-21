// LeetCode 2300 - Successful Pairs of Spells and Potions
// A spell/potion pair is successful if spell * potion >= success. For each
// spell return how many potions form a successful pair with it.
// Approach: sort potions; for each spell, lower_bound the first potion with
// product >= success.
// Time: O((n + m) log m), Space: O(1) besides sort/output

fn successful_pairs(spells: &[i32], mut potions: Vec<i32>, success: i64) -> Vec<usize> {
    potions.sort_unstable();
    spells
        .iter()
        .map(|&s| potions.len() - potions.partition_point(|&p| (p as i64) * (s as i64) < success))
        .collect()
}

fn main() {
    assert_eq!(successful_pairs(&[5, 1, 3], vec![1, 2, 3, 4, 5], 7), vec![4, 0, 3]);
    assert_eq!(successful_pairs(&[3, 1, 2], vec![8, 5, 8], 16), vec![2, 0, 2]);
    println!("ok");
}
