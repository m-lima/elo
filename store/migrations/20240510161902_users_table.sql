CREATE TABLE users (
  id         INTEGER NOT NULL PRIMARY KEY,
  email      TEXT    NOT NULL UNIQUE
    CHECK(LENGTH(TRIM(email)) > 0),
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4))
);
