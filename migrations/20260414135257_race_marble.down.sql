CREATE TABLE race_participant (
    id INTEGER NOT NULL,
    name TEXT NOT NULL,
    time REAL NOT NULL,
    PRIMARY KEY (id, name),
    FOREIGN KEY (id) REFERENCES race (id)
);

INSERT INTO race_participant
SELECT
    race_marble.race_id AS id,
    marble.name AS name,
    time,
FROM race_marble
INNER JOIN marble
ON race_marble.id = marble.id;

DROP TABLE IF EXISTS race_marble;
