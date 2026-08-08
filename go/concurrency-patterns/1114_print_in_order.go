//go:build ignore

// LeetCode 1114 - Print in Order
// Statement: Three goroutines call First(), Second(), Third() in arbitrary order.
// Ensure the output is always "firstsecondthird".
// Approach: two channels used as one-shot signals (closed when done); Second waits
// on firstDone, Third waits on secondDone.
// Time: O(1) per call. Space: O(1).
package main

import (
	"fmt"
	"strings"
	"sync"
)

type Foo struct{ firstDone, secondDone chan struct{} }

func NewFoo() *Foo { return &Foo{make(chan struct{}), make(chan struct{})} }

func (f *Foo) First(p func())  { p(); close(f.firstDone) }
func (f *Foo) Second(p func()) { <-f.firstDone; p(); close(f.secondDone) }
func (f *Foo) Third(p func())  { <-f.secondDone; p() }

func main() {
	for _, order := range [][]int{{1, 2, 3}, {3, 2, 1}, {2, 3, 1}} {
		f := NewFoo()
		var mu sync.Mutex
		var sb strings.Builder
		w := func(s string) func() { return func() { mu.Lock(); sb.WriteString(s); mu.Unlock() } }
		var wg sync.WaitGroup
		for _, o := range order {
			wg.Add(1)
			go func(o int) {
				defer wg.Done()
				switch o {
				case 1:
					f.First(w("first"))
				case 2:
					f.Second(w("second"))
				case 3:
					f.Third(w("third"))
				}
			}(o)
		}
		wg.Wait()
		fmt.Println(order, "->", sb.String())
	}
}
