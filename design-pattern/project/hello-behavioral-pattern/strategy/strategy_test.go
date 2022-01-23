package strategy

import "testing"

func TestAfter(t *testing.T) {
	lfu := NewLFU()
	cache := InitCache(lfu)

	cache.Add("a", "1")
	cache.Add("b", "2")
	cache.Add("c", "3")

	lru := NewLRU()
	cache.SetAlgo(lru)

	cache.Add("d", "4")

	fifo := NewFIFO()
	cache.SetAlgo(fifo)

	cache.Add("e", "5")
}
