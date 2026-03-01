// LeetCode 391 - Perfect Rectangle
// Given axis-aligned rectangles, return true if together they exactly cover
// a rectangular region with no overlaps and no gaps.
// Approach: areas must sum to the bounding box area, and corners seen an odd number of
// times must be exactly the four bounding-box corners.
// Time: O(n) | Space: O(n)
use std::collections::HashSet;

pub fn is_rectangle_cover(rects: &[[i64; 4]]) -> bool {
    let (mut x1, mut y1, mut x2, mut y2) = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    let mut area = 0;
    let mut corners = HashSet::new();
    for r in rects {
        x1 = x1.min(r[0]);
        y1 = y1.min(r[1]);
        x2 = x2.max(r[2]);
        y2 = y2.max(r[3]);
        area += (r[2] - r[0]) * (r[3] - r[1]);
        for c in [(r[0], r[1]), (r[0], r[3]), (r[2], r[1]), (r[2], r[3])] {
            if !corners.remove(&c) {
                corners.insert(c);
            }
        }
    }
    area == (x2 - x1) * (y2 - y1)
        && corners.len() == 4
        && [(x1, y1), (x1, y2), (x2, y1), (x2, y2)].iter().all(|c| corners.contains(c))
}

fn main() {
    assert!(is_rectangle_cover(&[[1, 1, 3, 3], [3, 1, 4, 2], [3, 2, 4, 4], [1, 3, 2, 4], [2, 3, 3, 4]]));
    assert!(!is_rectangle_cover(&[[1, 1, 2, 3], [1, 3, 2, 4], [3, 1, 4, 2], [3, 2, 4, 4]]));
    println!("ok");
}
