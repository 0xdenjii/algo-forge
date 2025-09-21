// Classic - Binary Search on Reals (cube root)
// Compute the real cube root of x (any sign) to absolute precision 1e-9
// without powf/cbrt.
// Approach: bisection on [min(-1, x), max(1, x)] for a fixed number of
// iterations (100 halvings is far beyond f64 precision), avoiding eps loops
// that can spin forever.
// Time: O(iterations), Space: O(1)

fn cube_root(x: f64) -> f64 {
    let (mut lo, mut hi) = (x.min(-1.0), x.max(1.0));
    for _ in 0..100 {
        let mid = (lo + hi) / 2.0;
        if mid * mid * mid < x { lo = mid } else { hi = mid }
    }
    (lo + hi) / 2.0
}

fn main() {
    for &(x, want) in &[(27.0, 3.0), (-8.0, -2.0), (0.001, 0.1), (2.0, 1.259_921_049_894_873)] {
        let got = cube_root(x);
        assert!((got - want).abs() < 1e-9, "cbrt({x}) = {got}");
    }
    println!("cbrt(10) = {:.9}", cube_root(10.0));
}
