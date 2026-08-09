//go:build ignore

// LeetCode 1188 - Design Bounded Blocking Queue
// Statement: Implement a thread-safe fixed-capacity queue: Enqueue blocks when full,
// Dequeue blocks when empty, Size returns the current count.
// Approach: mutex + two condition variables (notFull, notEmpty) over a ring buffer.
// Time: O(1) per op. Space: O(capacity).
package main

import (
	"fmt"
	"sync"
)

type BoundedBlockingQueue struct {
	mu                sync.Mutex
	notFull, notEmpty *sync.Cond
	buf               []int
	head, size        int
}

func New(capacity int) *BoundedBlockingQueue {
	q := &BoundedBlockingQueue{buf: make([]int, capacity)}
	q.notFull = sync.NewCond(&q.mu)
	q.notEmpty = sync.NewCond(&q.mu)
	return q
}

func (q *BoundedBlockingQueue) Enqueue(x int) {
	q.mu.Lock()
	for q.size == len(q.buf) {
		q.notFull.Wait()
	}
	q.buf[(q.head+q.size)%len(q.buf)] = x
	q.size++
	q.notEmpty.Signal()
	q.mu.Unlock()
}

func (q *BoundedBlockingQueue) Dequeue() int {
	q.mu.Lock()
	for q.size == 0 {
		q.notEmpty.Wait()
	}
	x := q.buf[q.head]
	q.head = (q.head + 1) % len(q.buf)
	q.size--
	q.notFull.Signal()
	q.mu.Unlock()
	return x
}

func (q *BoundedBlockingQueue) Size() int { q.mu.Lock(); defer q.mu.Unlock(); return q.size }

func main() {
	q := New(3)
	var wg sync.WaitGroup
	sum := 0
	wg.Add(1)
	go func() {
		defer wg.Done()
		for i := 0; i < 100; i++ {
			sum += q.Dequeue()
		}
	}()
	for p := 0; p < 4; p++ {
		wg.Add(1)
		go func(p int) {
			defer wg.Done()
			for i := 0; i < 25; i++ {
				q.Enqueue(p*25 + i)
			}
		}(p)
	}
	wg.Wait()
	fmt.Println("sum:", sum, "expected:", 99*100/2, "size:", q.Size())
}
