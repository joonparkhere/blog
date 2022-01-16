package CoR

type department interface {
	execute(*patient)
	setNext(department)
}
