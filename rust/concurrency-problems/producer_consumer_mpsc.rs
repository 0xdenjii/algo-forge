// Classic - Producer / Consumer with channels
// Several producers generate jobs; several consumers process them. Shut down cleanly
// once all producers are done, and collect every result exactly once.
// Approach: std::sync::mpsc::sync_channel (bounded, gives backpressure). Multiple
// consumers share the Receiver through Arc<Mutex<Receiver>>; dropping all Senders
// closes the channel so consumers exit their loop.
// Time: O(jobs). Space: O(channel bound).
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let (tx, rx) = mpsc::sync_channel::<u64>(8);
    let rx = Arc::new(Mutex::new(rx));
    let (res_tx, res_rx) = mpsc::channel::<u64>();

    let producers: Vec<_> = (0..3u64)
        .map(|p| {
            let tx = tx.clone();
            thread::spawn(move || {
                for i in 0..100 {
                    tx.send(p * 100 + i).unwrap();
                }
            })
        })
        .collect();
    drop(tx); // only producer clones remain

    let consumers: Vec<_> = (0..4)
        .map(|_| {
            let (rx, res_tx) = (rx.clone(), res_tx.clone());
            thread::spawn(move || loop {
                // lock only for the recv, release before processing
                let job = rx.lock().unwrap().recv();
                match job {
                    Ok(x) => res_tx.send(x * x).unwrap(),
                    Err(_) => break,
                }
            })
        })
        .collect();
    drop(res_tx);

    for p in producers {
        p.join().unwrap();
    }
    for c in consumers {
        c.join().unwrap();
    }
    let results: Vec<u64> = res_rx.iter().collect();
    assert_eq!(results.len(), 300);
    let expected: u64 = (0..300u64).map(|x| x * x).sum();
    assert_eq!(results.iter().sum::<u64>(), expected);
    println!("processed {} jobs, sum of squares = {}", results.len(), expected);
}
