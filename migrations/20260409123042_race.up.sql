CREATE TABLE race (
    id INTEGER NOT NULL PRIMARY KEY
);

CREATE TABLE race_participant (
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    time REAL NOT NULL,
    PRIMARY KEY (id, name),
    FOREIGN KEY (id) REFERENCES race (id)
);
