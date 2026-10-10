//go:build ignore

// Sleeping Barber Problem (Dijkstra)
// Statement: A barber shop has one barber and N waiting chairs. Arriving customers
// leave if all chairs are full; otherwise they wait. The barber sleeps when idle.
// Approach: buffered channel of size N is the waiting room; a non-blocking send models
// "leave if full"; the barber blocks on receive (sleeping) until a customer arrives.
// Time: O(customers). Space: O(N).
package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	const chairs, customers = 3, 12
	waiting := make(chan int, chairs)
	var served, turnedAway []int
	var mu sync.Mutex
	barberDone := make(chan struct{})
	go func() {
		defer close(barberDone)
		for c := range waiting { // blocks (sleeps) while no customers
			time.Sleep(3 * time.Millisecond) // haircut
			served = append(served, c)
		}
	}()
	for c := 1; c <= customers; c++ {
		select {
		case waiting <- c:
		default:
			mu.Lock()
			turnedAway = append(turnedAway, c)
			mu.Unlock()
		}
		time.Sleep(time.Millisecond)
	}
	close(waiting)
	<-barberDone
	fmt.Println("served:", served)
	fmt.Println("turned away:", turnedAway)
	fmt.Println("total accounted:", len(served)+len(turnedAway) == customers)
}
