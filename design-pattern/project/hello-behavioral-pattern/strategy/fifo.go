package strategy

import "fmt"

type fifo struct {
}

func NewFIFO() *fifo {
	return &fifo{}
}

func (l *fifo) Evict(c *Cache) {
	fmt.Println("Evicting by fifo strategy")
}
