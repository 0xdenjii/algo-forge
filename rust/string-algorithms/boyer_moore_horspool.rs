// Classic - Boyer-Moore-Horspool substring search
// Find all occurrences of a pattern in a text, comparing right-to-left inside each window.
// Approach: bad-character shift table keyed by the byte aligned with the window's last position;
// shift = m - 1 - (last index of that byte in pat[..m-1]), default m.
// Time: O(n/m) best, O(n*m) worst; Space: O(SIGMA)

pub fn horspool_find_all(text: &[u8], pat: &[u8]) -> Vec<usize> {
    let (n, m) = (text.len(), pat.len());
    if m == 0 || m > n {
        return vec![];
    }
    let mut shift = [m; 256];
    for (i, &c) in pat[..m - 1].iter().enumerate() {
        shift[c as usize] = m - 1 - i;
    }
    let mut res = Vec::new();
    let mut pos = 0;
    while pos + m <= n {
        let mut j = m;
        while j > 0 && text[pos + j - 1] == pat[j - 1] {
            j -= 1;
        }
        if j == 0 {
            res.push(pos);
        }
        pos += shift[text[pos + m - 1] as usize];
    }
    res
}

fn main() {
    assert_eq!(horspool_find_all(b"here is a simple example", b"example"), vec![17]);
    assert_eq!(horspool_find_all(b"aaaaa", b"aa"), vec![0, 1, 2, 3]);
    assert_eq!(horspool_find_all(b"abc", b"d"), Vec::<usize>::new());
    println!("ok");
}
