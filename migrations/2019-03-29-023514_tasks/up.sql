-- Your SQL goes here
CREATE TABLE tasks (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(1024) NOT NULL,
    description TEXT,
    category INTEGER NOT NULL,
    tasktype INTEGER NOT NULL,
    status INTEGER NOT NULL,
    unplanned INTEGER NOT NULL
)