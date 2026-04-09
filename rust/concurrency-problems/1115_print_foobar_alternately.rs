// LeetCode 1115 - Print FooBar Alternately
// Two threads: one prints "foo" n times, the other "bar" n times. Output must be
// "foobar" repeated n times.
// Approach: shared boolean turn flag guarded by Mutex + Condvar.
// Time: O(n). Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct FooBar {
    n: usize,
    foo_turn: Mutex<bool>,
    cv: Condvar,
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar { n, foo_turn: Mutex::new(true), cv: Condvar::new() }
    }
    fn step(&self, mine: bool, mut f: impl FnMut()) {
        for _ in 0..self.n {
            let mut t = self.cv.wait_while(self.foo_turn.lock().unwrap(), |t| *t != mine).unwrap();
            f();
            *t = !mine;
            self.cv.notify_one();
        }
    }
    fn foo(&self, f: impl FnMut()) { self.step(true, f) }
    fn bar(&self, f: impl FnMut()) { self.step(false, f) }
}

fn main() {
    let fb = Arc::new(FooBar::new(4));
    let out = Arc::new(Mutex::new(String::new()));
    let (a, oa) = (fb.clone(), out.clone());
    let t1 = thread::spawn(move || a.foo(|| oa.lock().unwrap().push_str("foo")));
    let (b, ob) = (fb.clone(), out.clone());
    let t2 = thread::spawn(move || b.bar(|| ob.lock().unwrap().push_str("bar")));
    t1.join().unwrap();
    t2.join().unwrap();
    assert_eq!(*out.lock().unwrap(), "foobar".repeat(4));
    println!("{}", out.lock().unwrap());
}
