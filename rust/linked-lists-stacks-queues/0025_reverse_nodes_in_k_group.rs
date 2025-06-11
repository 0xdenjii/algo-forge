// LeetCode 25 - Reverse Nodes in k-Group
// Reverse nodes k at a time; a trailing group shorter than k stays as is.
// Approach: verify k nodes exist, reverse that block, recurse on the remainder
// and attach it after the old block head (now the block tail).
// Time O(n), Space O(n/k) recursion.
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
    let mut p = &head;
    for _ in 0..k {
        match p { Some(n) => p = &n.next, None => return head }
    }
    let mut rest = head;
    let mut rev: Option<Box<ListNode>> = None;
    for _ in 0..k {
        let mut node = rest.take().unwrap();
        rest = node.next.take();
        node.next = rev;
        rev = Some(node);
    }
    let mut tail = &mut rev;
    while tail.as_ref().unwrap().next.is_some() { tail = &mut tail.as_mut().unwrap().next; }
    tail.as_mut().unwrap().next = reverse_k_group(rest, k);
    rev
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&reverse_k_group(build(&[1, 2, 3, 4, 5]), 2)), vec![2, 1, 4, 3, 5]);
    assert_eq!(to_vec(&reverse_k_group(build(&[1, 2, 3, 4, 5]), 3)), vec![3, 2, 1, 4, 5]);
    println!("ok");
}
