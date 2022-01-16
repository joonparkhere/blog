package abstract_factory

type iSportsFactory interface {
	makeShoe() iShoe
	makeShirt() iShirt
}

func getSportsFactory(brand string) iSportsFactory {
	if brand == "adidas" {
		return &adidas{}
	}

	if brand == "nike" {
		return &nike{}
	}

	return nil
}
