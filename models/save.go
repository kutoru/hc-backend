package models

import (
	"database/sql"
	"time"
)

type Save struct {
	ID       int
	Text     string
	Caption  string
	Filename *string
	Created  time.Time
}

func (save *Save) Scan(result *sql.Rows) error {
	return result.Scan(
		&save.ID,
		&save.Text,
		&save.Caption,
		&save.Filename,
		&save.Created,
	)
}
