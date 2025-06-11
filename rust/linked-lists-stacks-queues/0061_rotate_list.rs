// LeetCode 61 - Rotate List
// Rotate a list to the right by k places.
// Approach: compute length n, cut at index n - k % n, and append the old prefix
// after the detached suffix.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn rotate_right(mut head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let mut n = 0;
    let mut p = &head;
    while let Some(node) = p { n += 1; p = &node.next; }
    if n == 0 || k % n == 0 { return head; }
    let cut = n - k % n;
    let mut slot = &mut head;
    for _ in 0..cut { slot = &mut slot.as_mut().unwrap().next; }
    let mut new_head = slot.take();
    let mut tail = &mut new_head;
    while tail.is_some() { tail = &mut tail.as_mut().unwrap().next; }
    *tail = head;
    new_head
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&rotate_right(build(&[1, 2, 3, 4, 5]), 2)), vec![4, 5, 1, 2, 3]);
    assert_eq!(to_vec(&rotate_right(build(&[0, 1, 2]), 4)), vec![2, 0, 1]);
    println!("ok");
}
