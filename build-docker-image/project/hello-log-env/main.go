package main

import (
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"log"
	"net/http"
	"os"
)

func main() {
	server := echo.New()
	server.Use(middleware.Logger())
	server.Use(middleware.Recover())

	server.GET("/", func(c echo.Context) error {
		return c.JSON(http.StatusOK, "Hello Log Env")
	})

	server.GET("/env", func(c echo.Context) error {
		return c.JSON(http.StatusOK, os.Environ())
	})

	server.GET("/my-env", func(c echo.Context) error {
		return c.JSON(http.StatusOK, os.Getenv("MY_ENV"))
	})

	port := os.Getenv("PORT")
	log.Println("ENV PORT:", port)

	err := server.Start(":" + port)
	if err != nil {
		panic(err)
	}
}
