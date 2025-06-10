// LeetCode 23 - Merge k Sorted Lists
// Merge k sorted linked lists into one sorted list.
// Approach: min-heap of current heads (Reverse turns BinaryHeap into a min-heap).
// Time O(N log k), Space O(k).
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, PartialEq, Eq)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }
impl Ord for ListNode { fn cmp(&self, o: &Self) -> Ordering { self.val.cmp(&o.val) } }
impl PartialOrd for ListNode { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<Reverse<Box<ListNode>>> = lists.into_iter().flatten().map(Reverse).collect();
    let mut head = None;
    let mut tail = &mut head;
    while let Some(Reverse(mut node)) = heap.pop() {
        if let Some(next) = node.next.take() { heap.push(Reverse(next)); }
        tail = &mut tail.insert(node).next;
    }
    head
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    let m = merge_k_lists(vec![build(&[1, 4, 5]), build(&[1, 3, 4]), build(&[2, 6]), None]);
    assert_eq!(to_vec(&m), vec![1, 1, 2, 3, 4, 4, 5, 6]);
    println!("ok");
}
