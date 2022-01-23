package strategy

type Cache struct {
	storage     map[string]string
	algo        EvictionAlgo
	capacity    int
	maxCapacity int
}

func InitCache(a EvictionAlgo) *Cache {
	return &Cache{
		storage:     make(map[string]string),
		algo:        a,
		capacity:    0,
		maxCapacity: 2,
	}
}

func (c *Cache) SetAlgo(a EvictionAlgo) {
	c.algo = a
}

func (c *Cache) Add(key, value string) {
	if c.capacity == c.maxCapacity {
		c.evict()
	}

	c.capacity += 1
	c.storage[key] = value
}

func (c *Cache) Get(key string) string {
	value := c.storage[key]
	delete(c.storage, key)
	return value
}

func (c *Cache) evict() {
	c.algo.Evict(c)
	c.capacity -= 1
}
