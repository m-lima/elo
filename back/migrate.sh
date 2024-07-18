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

# Players
players=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM players;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","($4 == "" ? "NULL" : $4)","$5")," }')
# sqlite3 "${output}" "INSERT INTO players (id, name, email, inviter, created_ms) VALUES ${values%,};"

# Invites
invites=$(sqlite3 "${input}" 'SELECT id, name, email, inviter, created_ms FROM invites;' | awk -F'|' '{ print "("$1",\""$2"\",\""$3"\","$4","$5")," }')
# sqlite3 "${output}" "INSERT INTO invites (id, name, email, inviter, created_ms) VALUES ${values%,};"

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
