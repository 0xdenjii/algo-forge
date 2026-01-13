// LeetCode 6 - Zigzag Conversion
// Write s in a zigzag over num_rows rows and read it row by row.
// Approach: simulate by walking a row index that bounces between 0 and num_rows-1.
// Time: O(n), Space: O(n)

pub fn convert(s: &str, num_rows: usize) -> String {
    if num_rows <= 1 {
        return s.to_string();
    }
    let mut rows = vec![String::new(); num_rows.min(s.len().max(1))];
    let last = rows.len() - 1;
    let (mut r, mut down) = (0usize, true);
    for c in s.chars() {
        rows[r].push(c);
        if last == 0 {
            continue;
        }
        if r == 0 {
            down = true;
        } else if r == last {
            down = false;
        }
        r = if down { r + 1 } else { r - 1 };
    }
    rows.concat()
}

fn main() {
    assert_eq!(convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    assert_eq!(convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
    assert_eq!(convert("A", 1), "A");
    assert_eq!(convert("AB", 5), "AB");
    println!("ok");
}
