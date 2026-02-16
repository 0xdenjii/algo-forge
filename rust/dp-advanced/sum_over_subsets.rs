// Classic (Codeforces technique) - Sum over Subsets (SOS DP)
// Given a[mask] for all masks of b bits, compute f[mask] = sum of a[sub] over
// every submask sub of mask.
// Approach: iterate bits; for each bit i, add f[mask ^ (1<<i)] into f[mask]
// whenever bit i is set (a zeta transform over the subset lattice).
// Time: O(b * 2^b), Space: O(2^b)

pub fn sos(a: &[i64]) -> Vec<i64> {
    assert!(a.len().is_power_of_two());
    let bits = a.len().trailing_zeros();
    let mut f = a.to_vec();
    for i in 0..bits {
        for mask in 0..f.len() {
            if mask & (1 << i) != 0 {
                f[mask] += f[mask ^ (1 << i)];
            }
        }
    }
    f
}

fn brute(a: &[i64]) -> Vec<i64> {
    (0..a.len())
        .map(|m| (0..a.len()).filter(|&s| s & m == s).map(|s| a[s]).sum())
        .collect()
}

fn main() {
    let a: Vec<i64> = (0..16).map(|x| (x * 7 % 11) as i64).collect();
    assert_eq!(sos(&a), brute(&a));
    println!("{:?}", sos(&[1, 2, 3, 4]));
}
