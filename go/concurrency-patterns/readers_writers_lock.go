//go:build ignore

// Readers-Writers Problem (writer-preferring RW lock)
// Statement: Many readers may read shared data simultaneously, but a writer needs
// exclusive access; waiting writers must not be starved by a stream of readers.
// Approach: hand-rolled lock with mutex + cond: readers wait while a writer is active
// or waiting; a writer waits until no active readers and no active writer.
// Time: O(1) per op (excluding waits). Space: O(1).
package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

type RWLock struct {
	mu                                      sync.Mutex
	cond                                    *sync.Cond
	readers, writersWaiting                 int
	writing                                 bool
}

func NewRWLock() *RWLock { l := &RWLock{}; l.cond = sync.NewCond(&l.mu); return l }

func (l *RWLock) RLock() {
	l.mu.Lock()
	for l.writing || l.writersWaiting > 0 {
		l.cond.Wait()
	}
	l.readers++
	l.mu.Unlock()
}

func (l *RWLock) RUnlock() {
	l.mu.Lock()
	l.readers--
	if l.readers == 0 {
		l.cond.Broadcast()
	}
	l.mu.Unlock()
}

func (l *RWLock) Lock() {
	l.mu.Lock()
	l.writersWaiting++
	for l.writing || l.readers > 0 {
		l.cond.Wait()
	}
	l.writersWaiting--
	l.writing = true
	l.mu.Unlock()
}

func (l *RWLock) Unlock() {
	l.mu.Lock()
	l.writing = false
	l.cond.Broadcast()
	l.mu.Unlock()
}

func main() {
	l := NewRWLock()
	shared := map[string]int{}
	var activeReaders, activeWriters atomic.Int32
	var violations atomic.Int32
	var wg sync.WaitGroup
	for i := 0; i < 20; i++ {
		wg.Add(2)
		go func(i int) {
			defer wg.Done()
			l.Lock()
			if activeWriters.Add(1) != 1 || activeReaders.Load() != 0 {
				violations.Add(1)
			}
			shared["k"] += i
			activeWriters.Add(-1)
			l.Unlock()
		}(i)
		go func() {
			defer wg.Done()
			l.RLock()
			activeReaders.Add(1)
			if activeWriters.Load() != 0 {
				violations.Add(1)
			}
			_ = shared["k"]
			activeReaders.Add(-1)
			l.RUnlock()
		}()
	}
	wg.Wait()
	fmt.Println("final:", shared["k"], "expected:", 190, "violations:", violations.Load())
}
