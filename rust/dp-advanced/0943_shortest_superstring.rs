// LeetCode 943 - Find the Shortest Superstring
// Return the shortest string containing every given word as a substring
// (no word is a substring of another).
// Approach: Held-Karp style bitmask DP maximizing total overlap, then rebuild
// the order from parent pointers.
// Time: O(2^n * n^2), Space: O(2^n * n)

pub fn shortest_superstring(words: &[&str]) -> String {
    let n = words.len();
    let w: Vec<&[u8]> = words.iter().map(|s| s.as_bytes()).collect();
    let mut ov = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let m = w[i].len().min(w[j].len());
                ov[i][j] = (0..=m).rev().find(|&k| w[i].ends_with(&w[j][..k])).unwrap();
            }
        }
    }
    let full = 1 << n;
    let mut dp = vec![vec![-1i64; n]; full];
    let mut par = vec![vec![usize::MAX; n]; full];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for mask in 1..full {
        for last in 0..n {
            if dp[mask][last] < 0 {
                continue;
            }
            for nx in 0..n {
                if mask & (1 << nx) != 0 {
                    continue;
                }
                let nm = mask | (1 << nx);
                let v = dp[mask][last] + ov[last][nx] as i64;
                if v > dp[nm][nx] {
                    dp[nm][nx] = v;
                    par[nm][nx] = last;
                }
            }
        }
    }
    let mut mask = full - 1;
    let mut cur = (0..n).max_by_key(|&i| dp[mask][i]).unwrap();
    let mut order = Vec::with_capacity(n);
    while cur != usize::MAX {
        order.push(cur);
        let p = par[mask][cur];
        mask ^= 1 << cur;
        cur = p;
    }
    order.reverse();
    let mut res = String::from(words[order[0]]);
    for k in 1..n {
        res.push_str(&words[order[k]][ov[order[k - 1]][order[k]]..]);
    }
    res
}

fn main() {
    let r = shortest_superstring(&["catg", "ctaagt", "gcta", "ttca", "atgcatc"]);
    assert_eq!(r.len(), "gctaagttcatgcatc".len());
    for w in ["catg", "ctaagt", "gcta", "ttca", "atgcatc"] {
        assert!(r.contains(w));
    }
    assert_eq!(shortest_superstring(&["alex", "loves", "leetcode"]).len(), 17);
    println!("{r}");
}
