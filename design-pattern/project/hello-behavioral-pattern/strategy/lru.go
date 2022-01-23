package strategy

import "fmt"

type lru struct {
}

func NewLRU() *lru {
	return &lru{}
}

func (l *lru) Evict(c *Cache) {
	fmt.Println("Evicting by lru strategy")
}
