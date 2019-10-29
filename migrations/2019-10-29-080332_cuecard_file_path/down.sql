ALTER TABLE cuecards RENAME TO cuecards_drop;

CREATE TABLE cuecards (
	id INTEGER NOT NULL PRIMARY KEY,
	uuid TEXT NOT NULL UNIQUE,
	phase TEXT NOT NULL,
	rhythm TEXT NOT NULL,
	title TEXT NOT NULL,
	steplevel TEXT NOT NULL,
	difficulty TEXT NOT NULL,
	choreographer TEXT NOT NULL,
	meta TEXT NOT NULL,
	content TEXT NOT NULL,
    karaoke_marks TEXT NOT NULL DEFAULT '',
    music_file TEXT NOT NULL DEFAULT ''
);

INSERT INTO cuecards select id, uuid, phase, rhythm, title, steplevel, difficulty, choreographer, meta, content, karaoke_marks, music_file from cuecards_drop;
DROP TABLE cuecards_drop;