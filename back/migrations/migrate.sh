#!/usr/bin/env bash

if [ -z "${1}" ]; then
  echo 'Missing input file' >&2
  exit 1
elif [ -z "${2}" ]; then
  echo 'Missing output file' >&2
  exit 1
fi

input="${1}"
output="${2}"

function v0 {
  # Players
  players=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM players;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","($4 == "" ? "NULL" : $4)","$5")," }')

  # Invites
  invites=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM invites;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","$4","$5")," }')

  # Games
  games=$(sqlite3 "${input}" 'SELECT id, player_one, player_two, score_one, score_two, challenge, created_ms FROM games;' | awk -F'|' '{ print "("$1","$2","$3","$4","$5","$6","$7",0,0,0)," }')

  sqlite3 "${output}" <<EOF
BEGIN TRANSACTION;
INSERT INTO players (id, name, email, inviter, created_ms) VALUES ${players%,};
INSERT INTO invites (id, name, email, inviter, created_ms) VALUES ${invites%,};
INSERT INTO games (id, player_one, player_two, score_one, score_two, challenge, created_ms, rating_one, rating_two, rating_delta) VALUES ${games%,};
COMMIT;
EOF

  rm "${input}-shm" "${input}-wal" "${output}-shm" "${output}-wal" || true
}

function v1 {
  # Players
  players=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM players;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","($4 == "" ? "NULL" : $4)","$5")," }')

  # Invites
  invites=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM invites;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","$4","$5")," }')

  # Games
  games=$(sqlite3 "${input}" 'SELECT id, player_one, player_two, score_one, score_two, challenge, created_ms FROM games;' | awk -F'|' '{ print "("$1","$2","$3","$4","$5","$6","$7","$7",0,0,0,false)," }')

  sqlite3 "${output}" <<EOF
BEGIN TRANSACTION;
INSERT INTO players (id, name, email, inviter, created_ms) VALUES ${players%,};
INSERT INTO invites (id, name, email, inviter, created_ms) VALUES ${invites%,};
INSERT INTO games (id, player_one, player_two, score_one, score_two, challenge, created_ms, millis, rating_one, rating_two, rating_delta, deleted) VALUES ${games%,};
COMMIT;
EOF

  rm "${input}-shm" "${input}-wal" "${output}-shm" "${output}-wal" || true
}

function v2 {
  # Players
  players=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM players;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","($4 == "" ? "NULL" : $4)","$5")," }')

  # Invites
  invites=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM invites;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","$4","$5")," }')

  # Games
  games=$(sqlite3 "${input}" 'SELECT id, player_one, player_two, score_one, score_two, rating_one, rating_two, rating_delta, challenge, deleted, millis, created_ms FROM games;' | awk -F'|' '{ print "("$1","$2","$3","$4","$5","$6","$7","$8","$9","$10","$11","$12")," }')

  sqlite3 "${output}" <<EOF
BEGIN TRANSACTION;
INSERT INTO players (id, name, email, inviter, created_ms) VALUES ${players%,};
INSERT INTO invites (id, name, email, inviter, created_ms) VALUES ${invites%,};
INSERT INTO games (id, player_one, player_two, score_one, score_two, rating_one, rating_two, rating_delta, challenge, deleted, millis, created_ms) VALUES ${games%,};
COMMIT;
EOF

  rm "${input}-shm" "${input}-wal" "${output}-shm" "${output}-wal" || true
}

case ${3} in
  0) v0 ;;
  1) v1 ;;
  2) v2 ;;
  *)
    echo 'Missing version' >&2
    exit 1
    ;;
esac
