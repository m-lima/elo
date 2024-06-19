CREATE TABLE players (
  id          INTEGER NOT NULL PRIMARY KEY,
  name        TEXT    NOT NULL
    CHECK(LENGTH(TRIM(name)) > 0),
  email       TEXT    NOT NULL UNIQUE
    CHECK(LENGTH(TRIM(email)) > 0),
  inviter     INTEGER,

  -- Cached stats
  rating      REAL NOT NULL,
  wins        INTEGER NOT NULL DEFAULT 0,
  losses      INTEGER NOT NULL DEFAULT 0,
  points_won  INTEGER NOT NULL DEFAULT 0,
  points_lost INTEGER NOT NULL DEFAULT 0,

  created_ms  INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(inviter) REFERENCES players(id) ON DELETE SET NULL
);

CREATE TABLE invites (
  id         INTEGER NOT NULL PRIMARY KEY,
  inviter    INTEGER NOT NULL,
  name       TEXT    NOT NULL
    CHECK(LENGTH(TRIM(name)) > 0),
  email      TEXT    NOT NULL UNIQUE
    CHECK(LENGTH(TRIM(email) > 0)),
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(inviter) REFERENCES players(id) ON DELETE CASCADE
);

CREATE TABLE games (
  id         INTEGER NOT NULL PRIMARY KEY,
  player_one INTEGER NOT NULL,
  player_two INTEGER NOT NULL
    CHECK(player_one <> player_two),
  score_one  INTEGER NOT NULL,
  score_two  INTEGER NOT NULL
    CHECK(score_one <> score_two),
  rating_one REAL    NOT NULL,
  rating_two REAL    NOT NULL,
  accepted   BOOLEAN NOT NULL DEFAULT false,
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(player_one) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(player_two) REFERENCES players(id) ON DELETE CASCADE
);

CREATE TABLE challenges (
  id         INTEGER NOT NULL PRIMARY KEY,
  player_one INTEGER NOT NULL,
  player_two INTEGER NOT NULL
    CHECK(player_one <> player_two),
  game       INTEGER,
  refused    BOOLEAN NOT NULL DEFAULT false,
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(player_one) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(player_two) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(game)      REFERENCES games(id) ON DELETE SET NULL
);
