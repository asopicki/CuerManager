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
    music_file TEXT NOT NULL DEFAULT '',
    file_path TEXT NOT NULL DEFAULT '',
    date_created TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%dT%H:%M:%S.%fZ', 'now')),
    date_modified TEXT NOT NULL DEFAULT (STRFTIME('%Y-%m-%dT%H:%M:%S.%fZ', 'now'))
);

INSERT INTO cuecards select id, uuid, phase, rhythm, title, steplevel, difficulty, choreographer, meta, content, 
    karaoke_marks, music_file, file_path, STRFTIME('%Y-%m-%dT%H:%M:%S.%fZ', 'now') as date_created, 
    STRFTIME('%Y-%m-%dT%H:%M:%S.%fZ', 'now') as date_modified from cuecards_drop;
DROP TABLE cuecards_drop;

CREATE TRIGGER IF NOT EXISTS cuecards_bu BEFORE UPDATE ON cuecards BEGIN
  DELETE FROM cardindex WHERE docid=old.rowid;
END;
CREATE TRIGGER IF NOT EXISTS cuecards_bd BEFORE DELETE ON cuecards BEGIN
  DELETE FROM cardindex WHERE docid=old.rowid;
END;
CREATE TRIGGER IF NOT EXISTS cuecards_au AFTER UPDATE ON cuecards BEGIN
  INSERT INTO cardindex(docid, title, choreographer, meta, content) VALUES(new.rowid, new.title, new.choreographer,
  new.meta, new.content);
END;
CREATE TRIGGER IF NOT EXISTS cuecards_ai AFTER INSERT ON cuecards BEGIN
  INSERT INTO cardindex(docid, title, choreographer, meta, content) VALUES(new.rowid, new.title, new.choreographer,
  new.meta, new.content);
END;

ALTER TABLE tip_cuecards ADD cued_at TEXT DEFAULT NULL;