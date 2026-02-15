// LeetCode 1125 - Smallest Sufficient Team
// Given required skills and people (each with a skill list), return indices of
// a smallest team whose union of skills covers all required skills.
// Approach: bitmask DP over covered skills; dp[mask] = smallest team (stored as
// a u64 bitset of people) reaching mask.
// Time: O(2^s * p), Space: O(2^s)

use std::collections::HashMap;

pub fn smallest_sufficient_team(req: &[&str], people: &[Vec<&str>]) -> Vec<usize> {
    let idx: HashMap<&str, usize> = req.iter().enumerate().map(|(i, &s)| (s, i)).collect();
    let pm: Vec<usize> = people
        .iter()
        .map(|p| p.iter().filter_map(|s| idx.get(s)).fold(0, |m, &i| m | (1 << i)))
        .collect();
    let full = (1usize << req.len()) - 1;
    let mut dp: Vec<Option<u64>> = vec![None; full + 1];
    dp[0] = Some(0);
    for mask in 0..=full {
        let Some(team) = dp[mask] else { continue };
        for (i, &m) in pm.iter().enumerate() {
            let nm = mask | m;
            if nm == mask {
                continue;
            }
            let nt = team | (1u64 << i);
            if dp[nm].map_or(true, |t| t.count_ones() > nt.count_ones()) {
                dp[nm] = Some(nt);
            }
        }
    }
    let team = dp[full].expect("problem guarantees a solution");
    (0..people.len()).filter(|&i| team >> i & 1 == 1).collect()
}

fn main() {
    let req = ["java", "nodejs", "reactjs"];
    let people = vec![vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]];
    assert_eq!(smallest_sufficient_team(&req, &people), vec![0, 2]);
    println!("ok");
}
