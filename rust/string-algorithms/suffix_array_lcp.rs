// Classic - Suffix Array (prefix doubling) + LCP array (Kasai)
// Sort all suffixes of s lexicographically; lcp[i] = common prefix length of suffixes
// sa[i] and sa[i+1]. Demo: count distinct substrings = n(n+1)/2 - sum(lcp).
// Approach: rank doubling with sort_unstable_by_key on (rank[i], rank[i+k]); Kasai for LCP.
// Time: O(n log^2 n) build + O(n) LCP, Space: O(n)

pub fn suffix_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut sa: Vec<usize> = (0..n).collect();
    let mut rank: Vec<i64> = s.iter().map(|&c| c as i64).collect();
    let mut tmp = vec![0i64; n];
    let mut k = 1;
    while n > 1 {
        let key = |i: usize| (rank[i], if i + k < n { rank[i + k] } else { -1 });
        sa.sort_unstable_by_key(|&i| key(i));
        tmp[sa[0]] = 0;
        for w in 1..n {
            tmp[sa[w]] = tmp[sa[w - 1]] + (key(sa[w - 1]) < key(sa[w])) as i64;
        }
        std::mem::swap(&mut rank, &mut tmp);
        if rank[sa[n - 1]] as usize == n - 1 {
            break;
        }
        k <<= 1;
    }
    sa
}

pub fn lcp_kasai(s: &[u8], sa: &[usize]) -> Vec<usize> {
    let n = s.len();
    let mut rank = vec![0; n];
    for (i, &p) in sa.iter().enumerate() {
        rank[p] = i;
    }
    let mut lcp = vec![0; n.saturating_sub(1)];
    let mut h = 0usize;
    for i in 0..n {
        if rank[i] + 1 < n {
            let j = sa[rank[i] + 1];
            while i + h < n && j + h < n && s[i + h] == s[j + h] {
                h += 1;
            }
            lcp[rank[i]] = h;
            h = h.saturating_sub(1);
        } else {
            h = 0;
        }
    }
    lcp
}

pub fn distinct_substrings(s: &str) -> usize {
    let b = s.as_bytes();
    let sa = suffix_array(b);
    let n = b.len();
    n * (n + 1) / 2 - lcp_kasai(b, &sa).iter().sum::<usize>()
}

fn main() {
    let s = b"banana";
    let sa = suffix_array(s);
    assert_eq!(sa, vec![5, 3, 1, 0, 4, 2]);
    assert_eq!(lcp_kasai(s, &sa), vec![1, 3, 0, 0, 2]);
    assert_eq!(distinct_substrings("banana"), 15);
    assert_eq!(distinct_substrings("aaa"), 3);
    println!("ok");
}
