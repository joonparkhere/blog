package observer

type Subject interface {
	Join(Observer)
	Leave(Observer)
	NotifyAll()
}
