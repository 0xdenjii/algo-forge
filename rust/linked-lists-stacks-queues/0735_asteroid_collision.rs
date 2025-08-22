// LeetCode 735 - Asteroid Collision
// Positive asteroids move right, negative move left; on collision the smaller explodes
// (equal sizes both explode). Return the survivors.
// Approach: stack of survivors; a left-mover fights right-movers on top of the stack.
// Time O(n), Space O(n).
pub fn asteroid_collision(asteroids: &[i32]) -> Vec<i32> {
    let mut st: Vec<i32> = Vec::new();
    for &a in asteroids {
        let mut alive = true;
        while alive && a < 0 {
            match st.last() {
                Some(&top) if top > 0 => {
                    if top < -a { st.pop(); }
                    else { if top == -a { st.pop(); } alive = false; }
                }
                _ => break,
            }
        }
        if alive { st.push(a); }
    }
    st
}

fn main() {
    assert_eq!(asteroid_collision(&[5, 10, -5]), vec![5, 10]);
    assert_eq!(asteroid_collision(&[8, -8]), Vec::<i32>::new());
    assert_eq!(asteroid_collision(&[10, 2, -5]), vec![10]);
    assert_eq!(asteroid_collision(&[-2, -1, 1, 2]), vec![-2, -1, 1, 2]);
    println!("ok");
}
