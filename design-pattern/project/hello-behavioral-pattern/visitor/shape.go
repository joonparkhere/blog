package visitor

type Shape interface {
	GetType() string
	Accept(Visitor)
}
