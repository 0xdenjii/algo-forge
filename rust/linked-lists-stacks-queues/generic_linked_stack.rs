// Classic - Generic Singly Linked Stack (Too Many Lists style)
// Implement a generic stack backed by a singly linked list with push, pop, peek,
// peek_mut, borrowing iterator, IntoIterator, and a non-recursive Drop.
// Approach: Option<Box<Node<T>>> head; the iterative Drop avoids stack overflow on
// very long lists (default recursive drop would recurse once per node).
// Time O(1) per push/pop, Space O(n).
pub struct Stack<T> { head: Link<T>, len: usize }
type Link<T> = Option<Box<Node<T>>>;
struct Node<T> { elem: T, next: Link<T> }

impl<T> Stack<T> {
    pub fn new() -> Self { Self { head: None, len: 0 } }
    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node { elem, next: self.head.take() }));
        self.len += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| { self.head = n.next; self.len -= 1; n.elem })
    }
    pub fn peek(&self) -> Option<&T> { self.head.as_ref().map(|n| &n.elem) }
    pub fn peek_mut(&mut self) -> Option<&mut T> { self.head.as_mut().map(|n| &mut n.elem) }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn iter(&self) -> Iter<'_, T> { Iter { next: self.head.as_deref() } }
}

impl<T> Default for Stack<T> { fn default() -> Self { Self::new() } }

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut cur = self.head.take();
        while let Some(mut n) = cur { cur = n.next.take(); }
    }
}

pub struct Iter<'a, T> { next: Option<&'a Node<T>> }
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.next.map(|n| { self.next = n.next.as_deref(); &n.elem })
    }
}

pub struct IntoIter<T>(Stack<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.0.pop() }
}
impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> IntoIter<T> { IntoIter(self) }
}

fn main() {
    let mut s = Stack::new();
    for i in 1..=3 { s.push(i); }
    assert_eq!(s.iter().copied().collect::<Vec<_>>(), vec![3, 2, 1]);
    if let Some(top) = s.peek_mut() { *top *= 10; }
    assert_eq!(s.pop(), Some(30));
    assert_eq!(s.len(), 2);
    assert_eq!(s.into_iter().collect::<Vec<_>>(), vec![2, 1]);
    let mut big = Stack::new();
    for i in 0..1_000_000 { big.push(i); } // dropping must not overflow the stack
    println!("ok");
}
