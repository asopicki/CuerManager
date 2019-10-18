-- Your SQL goes here
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
	content TEXT NOT NULL
);

CREATE VIRTUAL TABLE cardindex USING fts4(title, choreographer, meta, content);

CREATE TRIGGER cuecards_bu BEFORE UPDATE ON cuecards BEGIN
  DELETE FROM cardindex WHERE docid=old.rowid;
END;
CREATE TRIGGER cuecards_bd BEFORE DELETE ON cuecards BEGIN
  DELETE FROM cardindex WHERE docid=old.rowid;
END;

CREATE TRIGGER cuecards_au AFTER UPDATE ON cuecards BEGIN
  INSERT INTO cardindex(docid, title, choreographer, meta, content) VALUES(new.rowid, new.title, new.choreographer,
  new.meta, new.content);
END;
CREATE TRIGGER cuecards_ai AFTER INSERT ON cuecards BEGIN
  INSERT INTO cardindex(docid, title, choreographer, meta, content) VALUES(new.rowid, new.title, new.choreographer,
  new.meta, new.content);
END;

CREATE TABLE playlists (
	id INTEGER NOT NULL PRIMARY KEY,
	uuid TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL
);

CREATE TABLE playlist_cuecards (
	id INTEGER NOT NULL PRIMARY KEY,
	playlist_id INTEGER NOT NULL,
	cuecard_id INTEGER NOT NULL
);