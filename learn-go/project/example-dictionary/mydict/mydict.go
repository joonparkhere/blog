package mydict

import "errors"

type Dictionary map[string]string

var (
	errNotFound = errors.New("Not found")
	errCannotUpdate = errors.New("Cannot update non-existing word")
	errWordExists = errors.New("That word already exists")
)

func (d Dictionary) Search(word string) (string, error) {
	value, exists := d[word]
	if (exists) {
		return value, nil
	}
	return "", errNotFound
}

func (d Dictionary) Add(word, definition string) error {
	_, err := d.Search(word)
	if err == errNotFound {
		d[word] = definition
		return nil
	}
	return errWordExists
}

func (d Dictionary) Update(word string, definition string) error {
	_, err := d.Search(word)
	switch err {
	case nil:
		d[word] = definition
	case errNotFound:
		return errCannotUpdate
	}
	return nil
}

func (d Dictionary) Delete(word string) {
	delete(d, word)
}
