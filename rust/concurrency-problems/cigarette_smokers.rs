// Classic - Cigarette Smokers problem (Patil)
// Three smokers each hold an infinite supply of one ingredient (tobacco, paper,
// matches). An agent repeatedly puts two random ingredients on the table; the smoker
// with the third must take them and smoke, then signal the agent.
// Approach: the table is a Mutex<Option<missing ingredient>>; each smoker waits for
// the round whose missing ingredient is its own. The agent waits for an empty table.
// Time: O(rounds). Space: O(1).
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

const NAMES: [&str; 3] = ["tobacco", "paper", "matches"];

struct Table {
    // Some(i): ingredients other than i are on the table. Stop when `done`.
    slot: Mutex<(Option<usize>, bool)>,
    cv: Condvar,
}

fn main() {
    let t = Arc::new(Table { slot: Mutex::new((None, false)), cv: Condvar::new() });
    let smokers: Vec<_> = (0..3)
        .map(|me| {
            let t = t.clone();
            thread::spawn(move || {
                let mut smoked = 0;
                loop {
                    let mut s = t.cv.wait_while(t.slot.lock().unwrap(), |s| s.0 != Some(me) && !s.1).unwrap();
                    if s.0 != Some(me) {
                        return smoked; // done
                    }
                    s.0 = None; // take the ingredients
                    smoked += 1;
                    t.cv.notify_all();
                }
            })
        })
        .collect();

    // simple LCG for deterministic "random" choices
    let mut seed: u64 = 42;
    let mut expected = [0; 3];
    for _ in 0..30 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let missing = (seed >> 33) as usize % 3;
        expected[missing] += 1;
        let mut s = t.cv.wait_while(t.slot.lock().unwrap(), |s| s.0.is_some()).unwrap();
        s.0 = Some(missing);
        t.cv.notify_all();
    }
    {
        let mut s = t.cv.wait_while(t.slot.lock().unwrap(), |s| s.0.is_some()).unwrap();
        s.1 = true;
        t.cv.notify_all();
    }
    for (i, h) in smokers.into_iter().enumerate() {
        let n = h.join().unwrap();
        assert_eq!(n, expected[i]);
        println!("smoker with {} smoked {} times", NAMES[i], n);
    }
}
