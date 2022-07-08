package main

import (
	"github.com/joonparkhere/study-project/Go/scrapper/scrap"
	"github.com/labstack/echo/v4"
	"os"
	"strings"
)

const fileName string = "jobs.csv"

func main()  {
	e := echo.New()
	e.GET("/", handleHome)
	e.POST("/scrap", handleScrap)
	e.Logger.Fatal(e.Start(":1323"))
}

func handleHome(context echo.Context) error {
	return context.File("home.html")
}

func handleScrap(context echo.Context) error {
	defer os.Remove(fileName)

	term := strings.ToLower(scrap.CleanString(context.FormValue("term")))
	scrap.Scrap(term)
	return context.Attachment(fileName, fileName)
}

