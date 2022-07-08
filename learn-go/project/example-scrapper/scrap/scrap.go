package scrap

import (
	"encoding/csv"
	"fmt"
	"github.com/PuerkitoBio/goquery"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
)

type extractedJob struct {
	id string
	location string
	title string
	salary string
	summary string
}

var jobViewBaseUrl string = "http://kr.indeed.com/viewjob?jk="
var baseUrl string

func Scrap(term string)  {
	baseUrl = "http://kr.indeed.com/jobs?q=" + term + "&limit=50"

	totalPages := getPages()
	fmt.Println("Number of total pages: ", totalPages)

	channel := make(chan []extractedJob)
	for i := 0; i < totalPages; i++ {
		go getPage(i, channel)
	}

	var jobs []extractedJob
	for i := 0; i < totalPages; i++ {
		extractedJobs := <- channel
		jobs = append(jobs, extractedJobs...)
	}

	writeJobs(jobs)
	fmt.Println("Done, extracted: ", len(jobs))
}

func getPage(page int, mainChannel chan<- []extractedJob) {
	pageUrl := baseUrl + "&start=" + strconv.Itoa(page * 50)

	fmt.Println("### Requesting ", pageUrl)
	res, err := http.Get(pageUrl)
	checkErr(err)
	checkCode(res)

	defer res.Body.Close()

	doc, err := goquery.NewDocumentFromReader(res.Body)
	checkErr(err)

	var jobs []extractedJob
	channel := make(chan extractedJob)
	searchCards := doc.Find(".jobsearch-SerpJobCard")
	searchCards.Each(func(i int, selection *goquery.Selection) {
		go extractJob(selection, channel)
	})

	for i := 0; i < searchCards.Length(); i++ {
		job := <- channel
		jobs = append(jobs, job)
	}
	mainChannel <- jobs
}

func extractJob(card *goquery.Selection, channel chan extractedJob) {
	id, _ := card.Attr("data-jk")
	title := CleanString(card.Find(".title>a").Text())
	location := CleanString(card.Find(".sjcl").Text())
	salary := CleanString(card.Find(".salaryText").Text())
	summary := CleanString(card.Find(".summary").Text())
	channel <- extractedJob{
		id: id,
		title: title,
		location: location,
		salary: salary,
		summary: summary,
	}
}

func writeJobs(jobs []extractedJob) {
	file, err := os.Create("jobs.csv")
	checkErr(err)

	writer := csv.NewWriter(file)
	defer writer.Flush()

	headers := []string{"Link", "Title", "Location", "Salary", "Summary"}

	writeErr := writer.Write(headers)
	checkErr(writeErr)

	for _, job := range jobs {
		jobSlice := []string{jobViewBaseUrl + job.id, job.title, job.location, job.salary, job.summary}
		jobWriteErr := writer.Write(jobSlice)
		checkErr(jobWriteErr)
	}
}

func CleanString(string string) string {
	return strings.Join(strings.Fields(strings.TrimSpace(string)), " ")
}

func getPages() int {
	pages := 0
	res, err := http.Get(baseUrl)
	checkErr(err)
	checkCode(res)

	defer res.Body.Close()

	doc, err := goquery.NewDocumentFromReader(res.Body)
	checkErr(err)

	doc.Find(".pagination").Each(func(i int, selection *goquery.Selection) {
		pages = selection.Find("a").Length()
	})
	return pages
}

func checkErr(err error) {
	if err != nil {
		log.Fatalln(err)
	}
}

func checkCode(res *http.Response) {
	if res.StatusCode != 200 {
		log.Fatalln("Request failed with status: ", res.StatusCode)
	}
}
