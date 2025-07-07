// LeetCode 148 - Sort List
// Sort a linked list in ascending order in O(n log n) time.
// Approach: top-down merge sort; split by length, sort halves, merge.
// Time O(n log n), Space O(log n) recursion.
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

fn len(mut l: &Option<Box<ListNode>>) -> usize {
    let mut n = 0; while let Some(x) = l { n += 1; l = &x.next; } n
}

fn merge(mut a: Option<Box<ListNode>>, mut b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    while let (Some(x), Some(y)) = (&a, &b) {
        let src = if x.val <= y.val { &mut a } else { &mut b };
        let mut node = src.take().unwrap();
        *src = node.next.take();
        tail = &mut tail.insert(node).next;
    }
    *tail = a.or(b);
    head
}

pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let n = len(&head);
    if n < 2 { return head; }
    let mut slot = &mut head;
    for _ in 0..n / 2 { slot = &mut slot.as_mut().unwrap().next; }
    let right = slot.take();
    merge(sort_list(head), sort_list(right))
}

fn build(v: &[i32]) -> Option<Box<ListNode>> {
    v.iter().rev().fold(None, |next, &val| Some(Box::new(ListNode { val, next })))
}
fn to_vec(mut l: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![]; while let Some(n) = l { out.push(n.val); l = &n.next; } out
}
fn main() {
    assert_eq!(to_vec(&sort_list(build(&[-1, 5, 3, 4, 0]))), vec![-1, 0, 3, 4, 5]);
    println!("ok");
}
