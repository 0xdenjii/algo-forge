// LeetCode 1195 - Fizz Buzz Multithreaded
// Four threads (fizz, buzz, fizzbuzz, number) must together print the FizzBuzz
// sequence for 1..=n in order.
// Approach: shared counter under a Mutex; each worker waits until the current value
// belongs to its category (or exceeds n, then exits), emits, increments, notifies all.
// Time: O(n) total work. Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

#[derive(Clone, Copy, PartialEq)]
enum Kind { Fizz, Buzz, FizzBuzz, Number }

fn kind(i: u32) -> Kind {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => Kind::FizzBuzz,
        (true, false) => Kind::Fizz,
        (false, true) => Kind::Buzz,
        _ => Kind::Number,
    }
}

struct FizzBuzz {
    n: u32,
    i: Mutex<u32>,
    cv: Condvar,
}

impl FizzBuzz {
    fn worker(&self, k: Kind, mut emit: impl FnMut(u32)) {
        loop {
            let mut i = self
                .cv
                .wait_while(self.i.lock().unwrap(), |i| *i <= self.n && kind(*i) != k)
                .unwrap();
            if *i > self.n {
                return;
            }
            emit(*i);
            *i += 1;
            self.cv.notify_all();
        }
    }
}

fn main() {
    let fb = Arc::new(FizzBuzz { n: 15, i: Mutex::new(1), cv: Condvar::new() });
    let out = Arc::new(Mutex::new(Vec::<String>::new()));
    let hs: Vec<_> = [Kind::Fizz, Kind::Buzz, Kind::FizzBuzz, Kind::Number]
        .into_iter()
        .map(|k| {
            let (fb, out) = (fb.clone(), out.clone());
            thread::spawn(move || {
                fb.worker(k, |i| {
                    let s = match k {
                        Kind::Fizz => "fizz".to_string(),
                        Kind::Buzz => "buzz".to_string(),
                        Kind::FizzBuzz => "fizzbuzz".to_string(),
                        Kind::Number => i.to_string(),
                    };
                    out.lock().unwrap().push(s);
                })
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    println!("{}", out.lock().unwrap().join(", "));
}
