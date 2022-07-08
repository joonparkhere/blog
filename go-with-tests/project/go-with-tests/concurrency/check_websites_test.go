package concurrency

import (
	"reflect"
	"testing"
)

func mockWebsiteChecker(url string) bool {
	if url == "https://www.acmicpc.net/" {
		return false
	}
	return true
}

func TestCheckWebsites(t *testing.T) {
	websites := []string{
		"https://www.google.co.kr/",
		"https://www.naver.com/",
		"https://www.acmicpc.net/",
	}

	want := map[string]bool{
		"https://www.google.co.kr/": true,
		"https://www.naver.com/": true,
		"https://www.acmicpc.net/": false,
	}

	got := CheckWebsites(mockWebsiteChecker, websites)

	if !reflect.DeepEqual(want, got) {
		t.Fatalf("Wanted %v, got %v", want, got)
	}
}