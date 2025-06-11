// LeetCode 24 - Swap Nodes in Pairs
// Swap every two adjacent nodes by relinking (values must not be modified).
// Approach: recursion; swap the first pair and attach the recursively swapped rest.
// Time O(n), Space O(n) recursion stack.
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(mut first) => match first.next.take() {
            Some(mut second) => {
                first.next = swap_pairs(second.next.take());
                second.next = Some(first);
                Some(second)
            }
            None => Some(first),
        },
        None => None,
    }
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&swap_pairs(build(&[1, 2, 3, 4, 5]))), vec![2, 1, 4, 3, 5]);
    println!("ok");
}
