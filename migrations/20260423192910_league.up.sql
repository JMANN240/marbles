CREATE TABLE
    league (
        id INTEGER NOT NULL PRIMARY KEY,
        name TEXT NOT NULL
    );

CREATE TABLE
    league_marble (
        league_id INTEGER NOT NULL,
        marble_id INTEGER NOT NULL,
        FOREIGN KEY (league_id) REFERENCES league (id) FOREIGN KEY (marble_id) REFERENCES marble (id)
    );

INSERT INTO
    league (id, name)
VALUES
    (1, "Major League"),
    (2, "Minor League"),
    (3, "Fan Leage");

INSERT INTO
    league_marble
VALUES
    (1, 2),
    (1, 3),
    (1, 4),
    (1, 5),
    (1, 6),
    (1, 7),
    (1, 8),
    (1, 9),
    (2, 10),
    (2, 11),
    (2, 12),
    (2, 13),
    (2, 14),
    (2, 15),
    (2, 16),
    (2, 17),
    (2, 18),
    (2, 19),
    (2, 20),
    (2, 21),
    (2, 22),
    (2, 23),
    (2, 24),
    (2, 25),
    (2, 26),
    (2, 29),
    (2, 33),
    (2, 39),
    (2, 40),
    (2, 41),
    (2, 42),
    (2, 44),
    (3, 1),
    (3, 27),
    (3, 28),
    (3, 30),
    (3, 31),
    (3, 32),
    (3, 34),
    (3, 35),
    (3, 36),
    (3, 37),
    (3, 38),
    (3, 43);