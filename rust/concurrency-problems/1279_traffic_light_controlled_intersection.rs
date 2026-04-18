// LeetCode 1279 - Traffic Light Controlled Intersection
// Road A (directions 1,2) and road B (directions 3,4) cross. Exactly one road is
// green at a time (initially A). A car arriving on a red road must turn its light
// green before crossing. Cars must never cross while their road is red.
// Approach: a Mutex holding the currently green road; carArrived holds the lock for
// the whole switch-then-cross, so no other car can flip the light mid-crossing.
// Time: O(1) per car. Space: O(1).
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Road { A, B }

struct TrafficLight {
    green: Mutex<Road>,
}

impl TrafficLight {
    fn new() -> Self {
        Self { green: Mutex::new(Road::A) }
    }
    fn car_arrived(&self, car_id: u32, road_id: u8, _direction: u8, turn_green: impl FnOnce(), cross_car: impl FnOnce()) {
        let road = if road_id == 1 { Road::A } else { Road::B };
        let mut g = self.green.lock().unwrap();
        if *g != road {
            turn_green();
            *g = road;
        }
        let _ = car_id;
        cross_car();
    }
}

fn main() {
    let tl = Arc::new(TrafficLight::new());
    let log = Arc::new(Mutex::new(Vec::<String>::new()));
    let cars = [(1, 1, 2), (3, 1, 1), (5, 2, 4), (7, 2, 3), (9, 1, 1)];
    let hs: Vec<_> = cars
        .into_iter()
        .map(|(id, road, dir)| {
            let (tl, log, l2) = (tl.clone(), log.clone(), log.clone());
            thread::spawn(move || {
                tl.car_arrived(
                    id, road, dir,
                    move || log.lock().unwrap().push(format!("Traffic light on road {} is green", if road == 1 { 'A' } else { 'B' })),
                    move || l2.lock().unwrap().push(format!("Car {id} has passed road {road} in direction {dir}")),
                )
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    for line in log.lock().unwrap().iter() {
        println!("{line}");
    }
}
