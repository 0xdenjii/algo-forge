// Classic - Segment Intersection Test
// Given two closed segments AB and CD with integer endpoints, decide whether they intersect
// (including touching and collinear overlap).
// Approach: orientation tests; general case uses opposite signs, collinear cases use bounding boxes.
// Time: O(1) | Space: O(1)
type P = (i64, i64);

fn orient(a: P, b: P, c: P) -> i64 {
    ((b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)).signum()
}

fn on_seg(a: P, b: P, c: P) -> bool {
    c.0 >= a.0.min(b.0) && c.0 <= a.0.max(b.0) && c.1 >= a.1.min(b.1) && c.1 <= a.1.max(b.1)
}

pub fn segments_intersect(a: P, b: P, c: P, d: P) -> bool {
    let (o1, o2, o3, o4) = (orient(a, b, c), orient(a, b, d), orient(c, d, a), orient(c, d, b));
    // General case: C and D on different sides of AB, and A and B on different sides of CD
    // (a zero means an endpoint touches the other segment's line, which still counts).
    if o1 != o2 && o3 != o4 {
        return true;
    }
    (o1 == 0 && on_seg(a, b, c))
        || (o2 == 0 && on_seg(a, b, d))
        || (o3 == 0 && on_seg(c, d, a))
        || (o4 == 0 && on_seg(c, d, b))
}

fn main() {
    assert!(segments_intersect((0, 0), (4, 4), (0, 4), (4, 0)));
    assert!(!segments_intersect((0, 0), (1, 1), (2, 2), (3, 3)));
    assert!(segments_intersect((0, 0), (2, 2), (1, 1), (3, 3)));
    assert!(segments_intersect((0, 0), (2, 0), (2, 0), (2, 5)));
    assert!(!segments_intersect((0, 0), (2, 0), (3, -1), (3, 1)));
    assert!(!segments_intersect((0, 0), (2, 0), (1, 1), (1, 3)));
    println!("ok");
}
