package main

import (
	"errors"
	"fmt"
	"net/http"
	"time"
)

type requestResult struct {
	url string
	status string
}

var (
	errRequestFailed = errors.New("Request failed")
)

func main() {
	urls := []string{
		"https://www.youtube.com/",
		"https://www.google.co.kr/",
		"https://www.acmicpc.net/",
		"https://papago.naver.com/",
		"https://keep.google.com/",
		"https://cs.skku.ac.kr/",
		"https://www.naver.com/",
		"https://www.inflearn.com/",
		"https://nomadcoders.co/",
		"https://www.myhome.go.kr/",
	}

	channel := make(chan requestResult)
	for _, url := range urls {
		go hitUrl(url, channel)
	}

	results := make(map[string]string)
	for i := 0; i < len(urls); i++ {
		result := <- channel
		results[result.url] = result.status
	}

	for url, status := range results {
		fmt.Println(url, status)
	}

	/*
		channel := make(chan string)
		people := []string{"seungho", "joon", "park", "noway"}
		for _, person := range people {
			go isMe(person, channel)
		}
		for i := 0; i < len(people); i++ {
			fmt.Println(<- channel)
		}
	*/
}

func hitUrl(url string, channel chan<- requestResult) {
	fmt.Println("CHECKING: ", url)
	response, error := http.Get(url)

	status := "OKAY"
	if error != nil || response.StatusCode >= 400 {
		status = "FAIL"
	}
	channel <- requestResult{url: url, status: status}
}

func isMe(person string, channel chan string) {
	time.Sleep(time.Second * 3)
	channel <- person + " is me"
}
