// LeetCode 284 - Peeking Iterator
// Wrap an existing iterator so it additionally supports peek() without advancing.
// Approach: generic adapter caching one look-ahead item in an Option; implements Iterator
// so it composes with the rest of std (this mirrors std::iter::Peekable).
// Time: O(1) per op. Space: O(1).
pub struct PeekingIterator<I: Iterator> {
    inner: I,
    peeked: Option<Option<I::Item>>,
}

impl<I: Iterator> PeekingIterator<I> {
    pub fn new(inner: I) -> Self {
        Self { inner, peeked: None }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        let inner = &mut self.inner;
        self.peeked.get_or_insert_with(|| inner.next()).as_ref()
    }

    pub fn has_next(&mut self) -> bool {
        self.peek().is_some()
    }
}

impl<I: Iterator> Iterator for PeekingIterator<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.inner.next(),
        }
    }
}

fn main() {
    let mut it = PeekingIterator::new(vec![1, 2, 3].into_iter());
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.peek(), Some(&2));
    assert_eq!(it.peek(), Some(&2));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert!(!it.has_next());
    assert_eq!(it.next(), None);

    // Use case: group consecutive equal items.
    let mut p = PeekingIterator::new("aaabccdd".chars());
    let mut runs = Vec::new();
    while let Some(c) = p.next() {
        let mut n = 1;
        while p.peek() == Some(&c) {
            p.next();
            n += 1;
        }
        runs.push((c, n));
    }
    assert_eq!(runs, vec![('a', 3), ('b', 1), ('c', 2), ('d', 2)]);
    println!("peeking iterator ok");
}
