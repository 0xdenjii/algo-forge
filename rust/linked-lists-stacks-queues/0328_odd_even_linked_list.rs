// LeetCode 328 - Odd Even Linked List
// Group all nodes at odd indices first, followed by nodes at even indices
// (1-indexed positions, relative order preserved).
// Approach: distribute nodes alternately onto two tail cursors, then join.
// Time O(n), Space O(1).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut odd, mut even) = (None, None);
    let (mut ot, mut et) = (&mut odd, &mut even);
    let mut is_odd = true;
    while let Some(mut node) = head {
        head = node.next.take();
        if is_odd { ot = &mut ot.insert(node).next; } else { et = &mut et.insert(node).next; }
        is_odd = !is_odd;
    }
    *ot = even;
    odd
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&odd_even_list(build(&[2, 1, 3, 5, 6, 4, 7]))), vec![2, 3, 6, 7, 1, 5, 4]);
    println!("ok");
}
