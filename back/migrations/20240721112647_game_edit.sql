CREATE TABLE history (
  id         INTEGER NOT NULL PRIMARY KEY,
  game       INTEGER NOT NULL,
  player_one INTEGER NOT NULL,
  player_two INTEGER NOT NULL,
  score_one  INTEGER NOT NULL,
  score_two  INTEGER NOT NULL,
  challenge  BOOLEAN NOT NULL,
  millis     INTEGER NOT NULL,
  deleted    BOOLEAN NOT NULL,
  created_ms INTEGER NOT NULL
    DEFAULT (strftime('%s', 'now') || substr(strftime('%f', 'now'), 4)),

  FOREIGN KEY(game) REFERENCES games(id) ON DELETE CASCADE
);

CREATE INDEX history_game_idx ON history(game);

CREATE TRIGGER games_before_update
  AFTER UPDATE ON games
  FOR EACH ROW
  WHEN
    NEW.id = OLD.id
    AND (
      NEW.player_one <> OLD.player_one
      OR NEW.player_two <> OLD.player_two
      OR NEW.score_one <> OLD.score_one
      OR NEW.score_two <> OLD.score_two
      OR NEW.challenge <> OLD.challenge
      OR NEW.deleted <> OLD.deleted
      OR NEW.millis <> OLD.millis
    )
BEGIN
  INSERT INTO history (
    game,
    player_one,
    player_two,
    score_one,
    score_two,
    challenge,
    deleted,
    millis
  ) VALUES (
    OLD.id,
    OLD.player_one,
    OLD.player_two,
    OLD.score_one,
    OLD.score_two,
    OLD.challenge,
    OLD.deleted,
    OLD.millis
  );
END;
