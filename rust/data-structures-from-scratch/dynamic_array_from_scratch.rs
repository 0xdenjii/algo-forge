// Classic - Dynamic Array (a minimal Vec<T>) from scratch
// Implement a growable array with push, pop, get, insert, remove, len, and correct Drop,
// managing raw memory directly.
// Approach: raw allocation via std::alloc with doubling capacity (amortized O(1) push);
// elements are moved with ptr::read/write/copy. Zero-sized types are rejected for simplicity.
// Time: push/pop amortized O(1), insert/remove O(n). Space: O(capacity).
use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "zero-sized types not supported");
        Self { ptr: NonNull::dangling(), cap: 0, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 4 } else { self.cap * 2 };
        let new_layout = Layout::array::<T>(new_cap).expect("capacity overflow");
        let raw = if self.cap == 0 {
            // SAFETY: layout has non-zero size (T is not a ZST and new_cap > 0).
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            // SAFETY: ptr was allocated with old_layout by this allocator.
            unsafe { alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) }
        };
        self.ptr = NonNull::new(raw as *mut T).unwrap_or_else(|| alloc::handle_alloc_error(new_layout));
        self.cap = new_cap;
    }

    pub fn push(&mut self, x: T) {
        if self.len == self.cap {
            self.grow();
        }
        // SAFETY: len < cap so the slot is inside the allocation and uninitialized.
        unsafe { ptr::write(self.ptr.as_ptr().add(self.len), x) };
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: slot `len` was initialized and is now logically removed.
        Some(unsafe { ptr::read(self.ptr.as_ptr().add(self.len)) })
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        // SAFETY: bounds-checked; slots below len are initialized.
        (i < self.len).then(|| unsafe { &*self.ptr.as_ptr().add(i) })
    }

    pub fn insert(&mut self, i: usize, x: T) {
        assert!(i <= self.len, "index out of bounds");
        if self.len == self.cap {
            self.grow();
        }
        // SAFETY: shift [i, len) right by one within capacity, then write.
        unsafe {
            let p = self.ptr.as_ptr().add(i);
            ptr::copy(p, p.add(1), self.len - i);
            ptr::write(p, x);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, i: usize) -> T {
        assert!(i < self.len, "index out of bounds");
        // SAFETY: read out slot i, then shift (i, len) left by one.
        unsafe {
            let p = self.ptr.as_ptr().add(i);
            let x = ptr::read(p);
            ptr::copy(p.add(1), p, self.len - i - 1);
            self.len -= 1;
            x
        }
    }

    pub fn as_slice(&self) -> &[T] {
        // SAFETY: ptr is non-null/aligned and the first len elements are initialized.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
        if self.cap != 0 {
            // SAFETY: matches the layout used for the current allocation.
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, Layout::array::<T>(self.cap).unwrap()) };
        }
    }
}

fn main() {
    let mut v = MyVec::new();
    for i in 0..100 {
        v.push(i);
    }
    assert_eq!(v.len(), 100);
    assert_eq!(v.pop(), Some(99));
    v.insert(0, -1);
    assert_eq!(v.get(0), Some(&-1));
    assert_eq!(v.remove(1), 0);
    assert_eq!(&v.as_slice()[..3], &[-1, 1, 2]);
    assert_eq!(v.get(1000), None);

    // Heap-owning elements must be dropped exactly once.
    let mut s = MyVec::new();
    for w in ["alpha", "beta", "gamma"] {
        s.push(w.to_string());
    }
    assert_eq!(s.remove(1), "beta");
    assert_eq!(s.as_slice(), &["alpha".to_string(), "gamma".to_string()]);
    println!("dynamic array ok");
}
