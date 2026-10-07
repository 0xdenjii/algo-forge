//go:build ignore

// Publish-Subscribe Broker
// Statement: Build an in-memory broker where subscribers register for topics and
// receive every message published to those topics; support unsubscribe and close.
// Approach: RWMutex-guarded map topic -> set of subscriber channels; Publish takes a
// read lock and sends to each (buffered) subscriber; Close closes all channels.
// Time: O(subscribers) per publish. Space: O(subscriptions).
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Broker struct {
	mu     sync.RWMutex
	subs   map[string]map[chan string]struct{}
	closed bool
}

func NewBroker() *Broker { return &Broker{subs: map[string]map[chan string]struct{}{}} }

func (b *Broker) Subscribe(topic string, buf int) chan string {
	b.mu.Lock()
	defer b.mu.Unlock()
	ch := make(chan string, buf)
	if b.subs[topic] == nil {
		b.subs[topic] = map[chan string]struct{}{}
	}
	b.subs[topic][ch] = struct{}{}
	return ch
}

func (b *Broker) Unsubscribe(topic string, ch chan string) {
	b.mu.Lock()
	defer b.mu.Unlock()
	if _, ok := b.subs[topic][ch]; ok {
		delete(b.subs[topic], ch)
		close(ch)
	}
}

func (b *Broker) Publish(topic, msg string) {
	b.mu.RLock()
	defer b.mu.RUnlock()
	if b.closed {
		return
	}
	for ch := range b.subs[topic] {
		ch <- msg
	}
}

func (b *Broker) Close() {
	b.mu.Lock()
	defer b.mu.Unlock()
	b.closed = true
	for _, set := range b.subs {
		for ch := range set {
			close(ch)
		}
	}
	b.subs = map[string]map[chan string]struct{}{}
}

func drain(name string, ch chan string, wg *sync.WaitGroup, out map[string][]string, mu *sync.Mutex) {
	defer wg.Done()
	for m := range ch {
		mu.Lock()
		out[name] = append(out[name], m)
		mu.Unlock()
	}
}

func main() {
	b := NewBroker()
	got := map[string][]string{}
	var mu sync.Mutex
	var wg sync.WaitGroup
	s1 := b.Subscribe("go", 10)
	s2 := b.Subscribe("go", 10)
	s3 := b.Subscribe("rust", 10)
	wg.Add(3)
	go drain("s1", s1, &wg, got, &mu)
	go drain("s2", s2, &wg, got, &mu)
	go drain("s3", s3, &wg, got, &mu)
	b.Publish("go", "goroutines")
	b.Unsubscribe("go", s2)
	b.Publish("go", "channels")
	b.Publish("rust", "ownership")
	b.Close()
	wg.Wait()
	keys := []string{"s1", "s2", "s3"}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Println(k, got[k])
	}
}
