-- Your SQL goes here
CREATE TABLE achievements (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    task INTEGER NOT NULL,
    date DATE NOT NULL,
    planned_time TIME,
    actual_time TIME,
    progress INTEGER NOT NULL,
    is_close_on INTEGER NOT NULL
)