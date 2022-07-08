package iteration

import "strings"

const repeatCount = 5

func Repeat(target string) string {
	var repeated string
	//for i := 0; i < repeatCount; i++ {
	//	repeated += target
	//}
	repeated = strings.Repeat(target, repeatCount)
	return repeated
}