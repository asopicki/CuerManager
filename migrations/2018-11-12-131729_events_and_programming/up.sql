-- Your SQL goes here

CREATE TABLE events (
    id INTEGER NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    date_start TEXT NOT NULL, -- ISO8601 date and time string
    date_end TEXT NOT NULL, -- ISO8601 date and time string
    name TEXT NOT NULL,
    schedule TEXT NULL,
    date_created TEXT NOT NULL,
    date_modified TEXT NOT NULL
);

CREATE TABLE programs (
    id INTEGER NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    notes TEXT,
    event_id INTEGER NOT NULL,
    date_created TEXT NOT NULL,
    date_modified TEXT NOT NULL
);

CREATE TABLE tips (
    id INTEGER NOT NULL PRIMARY KEY,
    uuid TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    program_id INTEGER NOT NULL,
    date_start TEXT NOT NULL,
    date_end TEXT NOT NULL
);

CREATE TABLE tip_cuecards (
	id INTEGER NOT NULL PRIMARY KEY,
	tip_id INTEGER NOT NULL,
	cuecard_id INTEGER NOT NULL,
    FOREIGN KEY (tip_id) REFERENCES tips(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (cuecard_id) REFERENCES cuecards(id)  ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY,
    tag TEXT NOT NULL
);

CREATE TABLE event_tags (
    id INTEGER NOT NULL PRIMARY KEY,
    event_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (tag_id)REFERENCES tags(id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE cuecard_tags (
    id INTEGER NOT NULL PRIMARY KEY, 
    cuecard_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (cuecard_id) REFERENCES cuecards(id)  ON UPDATE CASCADE ON DELETE CASCADE,
    FOREIGN KEY (tag_id)REFERENCES tags(id) ON UPDATE CASCADE ON DELETE CASCADE
);