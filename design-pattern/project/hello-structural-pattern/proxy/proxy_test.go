package proxy

import (
	"fmt"
	"testing"
)

func TestAfter(t *testing.T) {
	nginxServer := newNginxServer()
	appStatusURL := "/app/status"
	createUserURL := "/create/user"

	httpCode, body := nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(appStatusURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 200, Body: OK

	httpCode, body = nginxServer.handleRequest(createUserURL, "POST")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 201, Body: User Created

	httpCode, body = nginxServer.handleRequest(createUserURL, "GET")
	fmt.Printf("Url: %s, HttpCode: %d, Body: %s\n", appStatusURL, httpCode, body) // Url: /app/status, HttpCode: 404, Body: Not Ok
}
