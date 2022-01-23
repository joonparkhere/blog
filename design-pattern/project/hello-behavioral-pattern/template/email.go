package template

import (
	"fmt"
	"math/rand"
	"strconv"
)

type email struct {
}

func NewEmail() *email {
	return &email{}
}

func (e *email) GenRandomOTP(limit int) string {
	code := rand.Intn(limit)
	fmt.Printf("Email: generating random code %d\n", code)
	return strconv.Itoa(code)
}

func (e *email) SaveOTPCache(code string) {
	fmt.Printf("Eamil: saving code %s to cacahe\n", code)
}

func (e *email) GetMessage(code string) string {
	return "Email code for login is " + code
}

func (e *email) SendNotification(message string) error {
	fmt.Printf("Email: sending email %s\n", message)
	return nil
}

func (e *email) PublishMetric() {
	fmt.Println("Email: publishing metrics")
}
