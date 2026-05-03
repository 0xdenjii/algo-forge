// Lock-free - Test-and-Test-and-Set Spin Lock
// Implement a mutual-exclusion lock using only an atomic flag, with a guard that
// releases on drop, and show it protects a non-atomic counter.
// Approach: compare_exchange_weak(false -> true, Acquire) to lock; while contended,
// spin on a Relaxed load (TTAS) to avoid cache-line ping-pong; store(false, Release).
// Time: O(1) uncontended. Space: O(1).
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}
unsafe impl<T: Send> Sync for SpinLock<T> {}

pub struct Guard<'a, T>(&'a SpinLock<T>);

impl<T> SpinLock<T> {
    pub const fn new(v: T) -> Self {
        Self { locked: AtomicBool::new(false), data: UnsafeCell::new(v) }
    }
    pub fn lock(&self) -> Guard<'_, T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.locked.load(Ordering::Relaxed) {
                std::hint::spin_loop();
            }
        }
        Guard(self)
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.0.data.get() } }
}
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.0.data.get() } }
}
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.0.locked.store(false, Ordering::Release);
    }
}

fn main() {
    let l = Arc::new(SpinLock::new(Vec::new()));
    let hs: Vec<_> = (0..8)
        .map(|t| {
            let l = l.clone();
            thread::spawn(move || for i in 0..1000 { l.lock().push(t * 1000 + i); })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    assert_eq!(l.lock().len(), 8000);
    println!("pushed {} items", l.lock().len());
}
