package template

type iOTP interface {
	GenRandomOTP(int) string
	SaveOTPCache(string)
	GetMessage(string) string
	SendNotification(string) error
	PublishMetric()
}

type OTP struct {
	otp iOTP
}

func NewOTP(o iOTP) *OTP {
	return &OTP{
		otp: o,
	}
}

func (o *OTP) GenAndSendOTP(limit int) error {
	code := o.otp.GenRandomOTP(limit)
	o.otp.SaveOTPCache(code)

	message := o.otp.GetMessage(code)
	if err := o.otp.SendNotification(message); err != nil {
		return err
	}

	o.otp.PublishMetric()
	return nil
}
