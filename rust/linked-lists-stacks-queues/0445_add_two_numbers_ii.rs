// LeetCode 445 - Add Two Numbers II
// Digits are stored most-significant first; return the sum as a list, without
// reversing the inputs.
// Approach: push digits onto two stacks, pop and add, prepending each result node.
// Time O(n + m), Space O(n + m).
#[derive(Debug)]
pub struct ListNode { pub val: i32, pub next: Option<Box<ListNode>> }

fn digits(mut l: Option<Box<ListNode>>) -> Vec<i32> {
    let mut v = vec![];
    while let Some(n) = l { v.push(n.val); l = n.next; }
    v
}

pub fn add_two_numbers(a: Option<Box<ListNode>>, b: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut sa, mut sb) = (digits(a), digits(b));
    let mut carry = 0;
    let mut head = None;
    while !sa.is_empty() || !sb.is_empty() || carry > 0 {
        let sum = sa.pop().unwrap_or(0) + sb.pop().unwrap_or(0) + carry;
        carry = sum / 10;
        head = Some(Box::new(ListNode { val: sum % 10, next: head }));
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
    assert_eq!(to_vec(&add_two_numbers(build(&[7, 2, 4, 3]), build(&[5, 6, 4]))), vec![7, 8, 0, 7]);
    println!("ok");
}
