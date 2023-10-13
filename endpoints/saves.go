package endpoints

import (
	"fmt"
	"net/http"

	"github.com/kutoru/hc-backend/db"
	"github.com/kutoru/hc-backend/models"
)

func savesGet(w http.ResponseWriter, r *http.Request) {
	rows, err := db.Query("SELECT * FROM saves;")
	if err != nil {
		fmt.Println(err)
		http.Error(w, "Server error", http.StatusInternalServerError)
		return
	}

	saveList := make([]models.Save, 0)

	for rows.Next() {
		var save models.Save
		err = save.Scan(rows)
		if err != nil {
			fmt.Println("Error when scanning a save", err)
			continue
		}

		saveList = append(saveList, save)
	}

	fmt.Println(saveList)
	responseString := fmt.Sprintf("There are %v rows in the saves table", len(saveList))
	w.Write([]byte(responseString))
}

func savesPost(w http.ResponseWriter, r *http.Request) {
	_, err := db.Exec(`
		INSERT INTO saves (text, caption, filename) VALUES (?, ?, ?);
	`, "test text", "test caption", nil)
	if err != nil {
		fmt.Println(err)
		http.Error(w, "Server error", http.StatusInternalServerError)
		return
	}

	w.Write([]byte("+1 row success"))
}
