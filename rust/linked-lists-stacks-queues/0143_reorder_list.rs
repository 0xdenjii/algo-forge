// LeetCode 143 - Reorder List
// Reorder L0 -> L1 -> ... -> Ln into L0 -> Ln -> L1 -> Ln-1 -> ... in place.
// Approach: split at the middle, reverse the second half, then interleave.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut n = 0;
    let mut p = &*head;
    while let Some(node) = p { n += 1; p = &node.next; }
    if n < 3 { return; }
    // first half keeps ceil(n/2) nodes
    let mut slot = &mut *head;
    for _ in 0..(n + 1) / 2 { slot = &mut slot.as_mut().unwrap().next; }
    let mut second = slot.take();
    let mut rev = None;
    while let Some(mut node) = second { second = node.next.take(); node.next = rev; rev = Some(node); }
    // interleave
    let mut cur = head.as_mut();
    while let Some(node) = cur {
        if let Some(mut r) = rev.take() {
            rev = r.next.take();
            r.next = node.next.take();
            node.next = Some(r);
            cur = node.next.as_mut().unwrap().next.as_mut();
        } else {
            break;
        }
    }
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    let mut a = build(&[1, 2, 3, 4]);
    reorder_list(&mut a);
    assert_eq!(to_vec(&a), vec![1, 4, 2, 3]);
    let mut b = build(&[1, 2, 3, 4, 5]);
    reorder_list(&mut b);
    assert_eq!(to_vec(&b), vec![1, 5, 2, 4, 3]);
    println!("ok");
}
