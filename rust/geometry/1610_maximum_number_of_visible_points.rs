// LeetCode 1610 - Maximum Number of Visible Points
// Standing at `location`, you can see points whose polar angle fits in a window of `angle` degrees.
// Points at your location are always visible. Return the max number visible after rotating.
// Approach: sort polar angles, duplicate them shifted by 360, and slide a two-pointer window.
// Time: O(n log n) | Space: O(n)
pub fn visible_points(points: &[(i64, i64)], angle: f64, location: (i64, i64)) -> usize {
    let mut same = 0;
    let mut angs: Vec<f64> = Vec::new();
    for &(x, y) in points {
        let (dx, dy) = (x - location.0, y - location.1);
        if dx == 0 && dy == 0 {
            same += 1;
        } else {
            angs.push((dy as f64).atan2(dx as f64).to_degrees());
        }
    }
    angs.sort_by(|a, b| a.total_cmp(b));
    let n = angs.len();
    let ext: Vec<f64> = angs.iter().copied().chain(angs.iter().map(|a| a + 360.0)).collect();
    let (mut best, mut l) = (0, 0);
    for r in 0..ext.len() {
        while ext[r] - ext[l] > angle + 1e-9 {
            l += 1;
        }
        best = best.max((r - l + 1).min(n));
    }
    best + same
}

fn main() {
    assert_eq!(visible_points(&[(2, 1), (2, 2), (3, 3)], 90.0, (1, 1)), 3);
    assert_eq!(visible_points(&[(2, 1), (2, 2), (3, 4), (1, 1)], 90.0, (1, 1)), 4);
    assert_eq!(visible_points(&[(1, 0), (2, 1)], 13.0, (1, 1)), 1);
    println!("ok");
}
