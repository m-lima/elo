CREATE TABLE players (
  id          INTEGER NOT NULL PRIMARY KEY,
  name        TEXT    NOT NULL
    CHECK(LENGTH(TRIM(name)) > 0 AND LENGTH(name) <= 32),
  email       TEXT    NOT NULL UNIQUE
    CHECK(LENGTH(TRIM(email)) > 0 AND LENGTH(email) <= 128),
  inviter     INTEGER,
  created_ms  INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(inviter) REFERENCES players(id) ON DELETE SET NULL
);

CREATE TABLE invites (
  id         INTEGER NOT NULL PRIMARY KEY,
  inviter    INTEGER NOT NULL,
  name       TEXT    NOT NULL
    CHECK(LENGTH(TRIM(name)) > 0 AND LENGTH(name) <= 32),
  email      TEXT    NOT NULL UNIQUE
    CHECK(LENGTH(TRIM(email)) > 0 AND LENGTH(email) <= 128),
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(inviter) REFERENCES players(id) ON DELETE CASCADE
);

CREATE TABLE games (
  id           INTEGER NOT NULL PRIMARY KEY,
  player_one   INTEGER NOT NULL
    CHECK(player_one <> player_two),
  player_two   INTEGER NOT NULL
    CHECK(player_one <> player_two),
  score_one    INTEGER NOT NULL
    CHECK(
      (score_one = 11 AND score_two < 11)
      OR (score_one = 12 AND score_two = 10)
      OR (score_one < 11 AND score_two = 11)
      OR (score_one = 10 AND score_two = 12)
    ),
  score_two    INTEGER NOT NULL
    CHECK(
      (score_one = 11 AND score_two < 11)
      OR (score_one = 12 AND score_two = 10)
      OR (score_one < 11 AND score_two = 11)
      OR (score_one = 10 AND score_two = 12)
    ),
  rating_one   REAL    NOT NULL,
  rating_two   REAL    NOT NULL,
  rating_delta REAL    NOT NULL,
  challenge    BOOLEAN NOT NULL,
  created_ms   INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(player_one) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(player_two) REFERENCES players(id) ON DELETE CASCADE
);

CREATE INDEX games_player_one_idx ON games(player_one);
CREATE INDEX games_player_two_idx ON games(player_two);

CREATE TABLE challenges (
  id         INTEGER NOT NULL PRIMARY KEY,
  player_one INTEGER NOT NULL,
  player_two INTEGER NOT NULL
    CHECK(player_one <> player_two),
  game       INTEGER,
  refused    BOOLEAN NOT NULL DEFAULT FALSE,
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(player_one) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(player_two) REFERENCES players(id) ON DELETE CASCADE,
  FOREIGN KEY(game)      REFERENCES games(id) ON DELETE SET NULL
);
