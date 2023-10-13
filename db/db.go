package db

import (
	"database/sql"
	"fmt"
	"os"

	_ "github.com/mattn/go-sqlite3"
)

var db *sql.DB

func Initialize() {
	var err error
	db, err = sql.Open("sqlite3", "./hc.db")
	if err != nil {
		panic(err)
	}

	fmt.Println("Successfully initialized the DB")
}

func Reset() {
	script, err := os.ReadFile("./create_db.sql")
	if err != nil {
		panic(err)
	}

	_, err = db.Exec(string(script))
	if err != nil {
		panic(err)
	}

	fmt.Println("Successfully reset the DB")
}

func Query(query string, args ...any) (*sql.Rows, error) {
	return db.Query(query, args...)
}

func Exec(query string, args ...any) (sql.Result, error) {
	return db.Exec(query, args...)
}
