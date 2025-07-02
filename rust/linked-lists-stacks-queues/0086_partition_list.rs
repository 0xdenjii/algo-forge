// LeetCode 86 - Partition List
// Rearrange so all nodes < x come before nodes >= x, preserving relative order.
// Approach: split into two lists using tail cursors, then concatenate.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let (mut lo, mut hi) = (None, None);
    let (mut lt, mut ht) = (&mut lo, &mut hi);
    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x { lt = &mut lt.insert(node).next; } else { ht = &mut ht.insert(node).next; }
    }
    *lt = hi;
    lo
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&partition(build(&[1, 4, 3, 2, 5, 2]), 3)), vec![1, 2, 2, 4, 3, 5]);
    println!("ok");
}
