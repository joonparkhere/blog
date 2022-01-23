package template

import (
	"fmt"
	"testing"
)

func TestAfter(t *testing.T) {
	smsOTP := NewSMS()
	otp := NewOTP(smsOTP)
	otp.GenAndSendOTP(9999)

	fmt.Println()

	emailOTP := NewEmail()
	otp = NewOTP(emailOTP)
	otp.GenAndSendOTP(999999)
}
