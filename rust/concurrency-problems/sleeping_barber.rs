// Classic - Sleeping Barber (Dijkstra)
// A barber shop has one barber and N waiting chairs. If no customers, the barber
// sleeps. An arriving customer wakes him, waits in a free chair, or leaves if full.
// Approach: Mutex<VecDeque> as the waiting room plus a Condvar to wake the barber;
// a `closed` flag lets the barber finish the queue and go home.
// Time: O(1) per customer. Space: O(N) chairs.
use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct Shop {
    room: Mutex<(VecDeque<u32>, bool)>, // (waiting customers, closed)
    cv: Condvar,
    chairs: usize,
}

impl Shop {
    /// Returns true if the customer got a seat.
    fn arrive(&self, id: u32) -> bool {
        let mut r = self.room.lock().unwrap();
        if r.0.len() >= self.chairs {
            return false;
        }
        r.0.push_back(id);
        self.cv.notify_one();
        true
    }
    fn close(&self) {
        self.room.lock().unwrap().1 = true;
        self.cv.notify_all();
    }
    fn barber(&self) -> Vec<u32> {
        let mut served = vec![];
        loop {
            let mut r = self.cv.wait_while(self.room.lock().unwrap(), |r| r.0.is_empty() && !r.1).unwrap();
            match r.0.pop_front() {
                Some(c) => {
                    drop(r);
                    thread::sleep(Duration::from_millis(2)); // cutting hair
                    served.push(c);
                }
                None => return served, // closed and empty
            }
        }
    }
}

fn main() {
    let shop = Arc::new(Shop { room: Mutex::new((VecDeque::new(), false)), cv: Condvar::new(), chairs: 3 });
    let s = shop.clone();
    let barber = thread::spawn(move || s.barber());
    let mut turned_away = 0;
    for id in 0..20 {
        if !shop.arrive(id) {
            turned_away += 1;
        }
        thread::sleep(Duration::from_millis(1));
    }
    shop.close();
    let served = barber.join().unwrap();
    assert_eq!(served.len() + turned_away, 20);
    println!("served {:?}, turned away {}", served, turned_away);
}
