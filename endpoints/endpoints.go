package endpoints

import "github.com/gorilla/mux"

func LoadRouter() *mux.Router {
	r := mux.NewRouter()

	r.HandleFunc("/saves", savesGet).Methods("GET")
	r.HandleFunc("/saves", savesPost).Methods("POST")

	return r
}
