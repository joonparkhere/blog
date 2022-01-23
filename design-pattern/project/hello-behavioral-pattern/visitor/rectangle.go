package visitor

type Rectangle struct {
	height int
	width  int
}

func NewRectangle(h, w int) *Rectangle {
	return &Rectangle{
		height: h,
		width:  w,
	}
}

func (r *Rectangle) Accept(v Visitor) {
	v.VisitForRectangle(r)
}

func (r *Rectangle) GetType() string {
	return "Rectangle"
}
