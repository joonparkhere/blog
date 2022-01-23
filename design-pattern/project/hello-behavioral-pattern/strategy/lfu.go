package strategy

import "fmt"

type lfu struct {
}

func NewLFU() *lfu {
	return &lfu{}
}

func (l *lfu) Evict(c *Cache) {
	fmt.Println("Evicting by lfu strategy")
}
