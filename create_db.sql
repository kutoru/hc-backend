DROP TABLE IF EXISTS notes;
DROP TABLE IF EXISTS note_categories;
DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS saves;

CREATE TABLE saves (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    text TEXT NOT NULL,
    caption TEXT NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE files (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    save_id INTEGER NOT NULL,
    hash_name TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (save_id) REFERENCES saves(id)
);

CREATE TABLE note_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    category_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    text TEXT NOT NULL,
    created DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (category_id) REFERENCES note_categories(id)
);
