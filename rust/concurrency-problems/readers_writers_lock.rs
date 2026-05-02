// Classic - Readers-Writers problem (writer-preferring)
// Many readers may hold the resource simultaneously; a writer needs exclusive access.
// Prevent writer starvation: once a writer is waiting, new readers must queue.
// Approach: hand-rolled RW lock from Mutex + Condvar tracking active readers, an
// active-writer flag and the number of waiting writers. RAII guards release it.
// Time: O(1) per acquire/release (excluding waiting). Space: O(1).
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Default)]
struct St {
    readers: usize,
    writer: bool,
    waiting_writers: usize,
}

pub struct RwLock<T> {
    st: Mutex<St>,
    cv: Condvar,
    data: UnsafeCell<T>,
}
// Safety: access to `data` is mediated by the reader/writer protocol below.
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

pub struct ReadGuard<'a, T>(&'a RwLock<T>);
pub struct WriteGuard<'a, T>(&'a RwLock<T>);

impl<T> RwLock<T> {
    pub fn new(v: T) -> Self {
        Self { st: Mutex::new(St::default()), cv: Condvar::new(), data: UnsafeCell::new(v) }
    }
    pub fn read(&self) -> ReadGuard<'_, T> {
        let mut s = self.cv.wait_while(self.st.lock().unwrap(), |s| s.writer || s.waiting_writers > 0).unwrap();
        s.readers += 1;
        ReadGuard(self)
    }
    pub fn write(&self) -> WriteGuard<'_, T> {
        let mut s = self.st.lock().unwrap();
        s.waiting_writers += 1;
        let mut s = self.cv.wait_while(s, |s| s.writer || s.readers > 0).unwrap();
        s.waiting_writers -= 1;
        s.writer = true;
        WriteGuard(self)
    }
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.0.data.get() } }
}
impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        let mut s = self.0.st.lock().unwrap();
        s.readers -= 1;
        if s.readers == 0 { self.0.cv.notify_all(); }
    }
}
impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T { unsafe { &*self.0.data.get() } }
}
impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.0.data.get() } }
}
impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.0.st.lock().unwrap().writer = false;
        self.0.cv.notify_all();
    }
}

fn main() {
    let lock = Arc::new(RwLock::new(0u64));
    let mut hs = vec![];
    for _ in 0..4 {
        let l = lock.clone();
        hs.push(thread::spawn(move || for _ in 0..1000 { *l.write() += 1; }));
    }
    for _ in 0..8 {
        let l = lock.clone();
        hs.push(thread::spawn(move || for _ in 0..1000 { let v = *l.read(); assert!(v <= 4000); }));
    }
    for h in hs { h.join().unwrap(); }
    assert_eq!(*lock.read(), 4000);
    println!("final = {}", *lock.read());
}
