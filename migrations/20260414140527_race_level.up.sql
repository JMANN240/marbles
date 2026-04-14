ALTER TABLE race ADD COLUMN level_id INTEGER NOT NULL DEFAULT 0;

WITH average_race_time AS (
    SELECT race.id, AVG(time) AS average_time
    FROM race
    INNER JOIN race_marble
    ON race.id = race_marble.race_id
    GROUP BY race.id
)
UPDATE race
SET level_id = CASE WHEN average_time < 20 THEN 11 ELSE 10 END
FROM average_race_time
WHERE race.id = average_race_time.id;
