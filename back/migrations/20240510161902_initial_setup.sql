CREATE TABLE players (
  id         INTEGER NOT NULL PRIMARY KEY,
  name       TEXT    NOT NULL
    CHECK(LENGTH(TRIM(name)) > 0),
  email      TEXT    NOT NULL UNIQUE
    CHECK(LENGTH(TRIM(email)) > 0),
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4))
);

-- Separate rating table so we can change the algorithm and replay the games
CREATE TABLE ratings (
  player     INTEGER NOT NULL PRIMARY KEY,
  rating     REAL NOT NULL,
  deviation  REAL NOT NULL,
  volatility REAL NOT NULL,

  FOREIGN KEY(player) REFERENCES players(id) ON DELETE CASCADE
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

CREATE TABLE matches (
  id         INTEGER NOT NULL PRIMARY KEY,
  player_one INTEGER NOT NULL,
  player_two INTEGER NOT NULL
    CHECK(player_one <> player_two),
  score_one  INTEGER NOT NULL,
  score_two  INTEGER NOT NULL,
  accepted   BOOLEAN NOT NULL DEFAULT false,
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(player_one) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(player_two) REFERENCES players(id) ON DELETE CASCADE
);

-- To be recalculated every time a match is accepted
CREATE TABLE stats_cache (
  player      INTEGER NOT NULL PRIMARY KEY,
  wins        INTEGER NOT NULL,
  losses      INTEGER NOT NULL,
  points_won  INTEGER NOT NULL,
  points_lost INTEGER NOT NULL,

  FOREIGN KEY(player) REFERENCES players(id) ON DELETE CASCADE
);

CREATE TABLE challenges (
  id         INTEGER NOT NULL PRIMARY KEY,
  player_one INTEGER NOT NULL,
  player_two INTEGER NOT NULL
    CHECK(player_one <> player_two),
  match      INTEGER,
  refused    BOOLEAN NOT NULL DEFAULT false,
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(player_one) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(player_two) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(match)      REFERENCES matches(id) ON DELETE SET NULL
);
