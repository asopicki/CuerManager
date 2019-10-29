ALTER TABLE tip_cuecards RENAME TO tip_cuecards_drop;

CREATE TABLE tip_cuecards (
	id INTEGER NOT NULL PRIMARY KEY,
	tip_id INTEGER NOT NULL,
	cuecard_id INTEGER NOT NULL,
    FOREIGN KEY (tip_id) REFERENCES tips(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (cuecard_id) REFERENCES cuecards(id)  ON UPDATE CASCADE ON DELETE CASCADE
);

INSERT INTO tip_cuecards select id, tip_id, cuecard_id from tip_cuecards_drop;
DROP TABLE tip_cuecards_drop;