//go:build ignore

// LeetCode 1279 - Traffic Light Controlled Intersection
// Statement: Road A (directions 1,2) and road B (3,4) cross; the light starts green on
// A. CarArrived must switch the light to the car's road if needed, then cross.
// Approach: one mutex serialises arrivals; remember the green road and call
// turnGreen only when it changes.
// Time: O(1) per car. Space: O(1).
package main

import (
	"fmt"
	"sync"
)

type TrafficLight struct {
	mu    sync.Mutex
	green int // 1 = road A, 2 = road B
}

func (t *TrafficLight) CarArrived(carID, roadID, direction int, turnGreen, crossCar func()) {
	t.mu.Lock()
	defer t.mu.Unlock()
	if t.green != roadID {
		turnGreen()
		t.green = roadID
	}
	crossCar()
}

func main() {
	t := &TrafficLight{green: 1}
	cars := []struct{ id, road, dir int }{{1, 1, 2}, {3, 2, 4}, {5, 1, 1}, {2, 2, 3}, {4, 2, 4}}
	for _, c := range cars {
		t.CarArrived(c.id, c.road, c.dir,
			func() { fmt.Printf("light green on road %d\n", c.road) },
			func() { fmt.Printf("car %d crosses road %d dir %d\n", c.id, c.road, c.dir) })
	}
	switches := 0
	var wg sync.WaitGroup
	for i := 0; i < 50; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			t.CarArrived(i, i%2+1, i%4+1, func() { switches++ }, func() {})
		}(i)
	}
	wg.Wait()
	fmt.Println("50 concurrent cars crossed; light switches:", switches)
}
