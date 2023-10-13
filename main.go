package main

import (
	"fmt"
	"net/http"
	"time"

	"github.com/gorilla/mux"
	"github.com/kutoru/hc-backend/db"
	"github.com/kutoru/hc-backend/endpoints"
)

func main() {
	db.Initialize()
	db.Reset()

	r := endpoints.LoadRouter()
	http.Handle("/", &RouterWrapper{Router: r})

	port := ":7272"
	fmt.Println(http.ListenAndServe(port, nil))
}

type RouterWrapper struct {
	Router *mux.Router
}

func (routerWrapper *RouterWrapper) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.URL.String() != "/favicon.ico" {
		now := time.Now().Format("2006-01-02 15:04:05")
		fmt.Printf("\n%v:\n%v %v\n", now, r.Method, r.URL)
	}

	routerWrapper.Router.ServeHTTP(w, r)
}
