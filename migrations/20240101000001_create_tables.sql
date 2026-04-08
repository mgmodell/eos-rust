CREATE TABLE IF NOT EXISTS chapter (
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS line (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    body TEXT NOT NULL,
    sequence INTEGER NOT NULL DEFAULT 0,
    chapter_id INTEGER NOT NULL,
    FOREIGN KEY (chapter_id) REFERENCES chapter(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS thread (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS chapter_depends_on_chapter (
    parent_chapter_id INTEGER NOT NULL,
    child_chapter_id INTEGER NOT NULL,
    thread_id INTEGER NOT NULL,
    PRIMARY KEY (parent_chapter_id, child_chapter_id, thread_id),
    FOREIGN KEY (parent_chapter_id) REFERENCES chapter(id) ON DELETE CASCADE,
    FOREIGN KEY (child_chapter_id) REFERENCES chapter(id) ON DELETE CASCADE,
    FOREIGN KEY (thread_id) REFERENCES thread(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS character (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS character_to_line (
    character_id INTEGER NOT NULL,
    line_id INTEGER NOT NULL,
    PRIMARY KEY (character_id, line_id),
    FOREIGN KEY (character_id) REFERENCES character(id) ON DELETE CASCADE,
    FOREIGN KEY (line_id) REFERENCES line(id) ON DELETE CASCADE
);
