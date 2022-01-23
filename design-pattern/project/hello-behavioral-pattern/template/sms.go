package template

import (
	"fmt"
	"math/rand"
	"strconv"
)

type sms struct {
}

func NewSMS() *sms {
	return &sms{}
}

func (s *sms) GenRandomOTP(limit int) string {
	code := rand.Intn(limit)
	fmt.Printf("SMS: generating random code %d\n", code)
	return strconv.Itoa(code)
}

func (s *sms) SaveOTPCache(code string) {
	fmt.Printf("SMS: saving code %s to cache\n", code)
}

func (s *sms) GetMessage(code string) string {
	return "SMS Code for login is " + code
}

func (s *sms) SendNotification(message string) error {
	fmt.Printf("SMS: sending sms %s\n", message)
	return nil
}

func (s *sms) PublishMetric() {
	fmt.Println("SMS: publishing metrics")
}
