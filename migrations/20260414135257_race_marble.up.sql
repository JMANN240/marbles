CREATE TABLE race_marble (
    race_id INTEGER NOT NULL,
    marble_id INTEGER NOT NULL,
    time REAL NOT NULL,
    place INTEGER NOT NULL,
    PRIMARY KEY (race_id, marble_id),
    FOREIGN KEY (race_id) REFERENCES race (id),
    FOREIGN KEY (marble_id) REFERENCES marble (id)
);

INSERT INTO race_marble
SELECT
    race_participant.id AS race_id,
    marble.id AS marble_id,
    time,
    RANK() OVER (PARTITION BY race_participant.id ORDER BY time, RANDOM()) as place
FROM race_participant
INNER JOIN marble
ON race_participant.name = marble.name;

DROP TABLE IF EXISTS race_participant;