#!/bin/bash

if [ ! "${1}" ]; then
  echo 'No database provided' >&2
  exit 1
fi

if [ ! -f "${1}" ]; then
  echo 'Path provided is not a file' >&2
  exit 1
fi

names=( "Gunther Balarama" "Gwythyr Odysseus" "Elli" "Nanna Hephaestus" "Kreios Kaleva" "Halcyone" "Devi Flora" "Aeneas Phrixos Nokomis" "Amen Seth" "Atlas Medrod" "Dylan" "Pangu Phoibe" "Vayu Baal" "Zababa" "Alf Chryses" "Tahmina Marduk" "Ormazd" "Daidalos Alastor" "Cupido Guendoleu" "Heiðrún" )

if [ "${2}" ]; then
  max=$(( ${2} + 0 ))
else
  max=${#names[@]}
fi
i

sqlite3 "${1}" 'insert into players ( name, email, rating, deviation, volatility ) values ( "Myself", "me@email.com", 1500, 0, 0 ) returning id, email, created_ms;'

for (( i = 0; i < ${max}; i++ )); do
  name="${names[$i]}"
  email=$(echo -n "${names[$i]}" | tr '[:upper:]' '[:lower:]' | tr ' ' '.')
  rating=$(( 1000 + ( RANDOM % 1000 ) ))
  sqlite3 "${1}" "insert into players ( name, email, rating, deviation, volatility ) values ( \"${name}\", \"${email}@email.com\", ${rating}, 0, 0 );"
done

if [ "${3}" ]; then
  maxGames=$(( ${3} + 0 ))
else
  maxGames=$(( max * 10 ))
fi

for (( i = 0; i < ${maxGames}; i++ )); do
  player_one=$(( 1 + RANDOM % max ))
  player_two=$(( 1 + RANDOM % max ))

  while (( player_one == player_two )); do
    player_two=$(( RANDOM % max ))
  done

  loser_score=$(( RANDOM % 10 ))

  if (( RANDOM % 2 )); then
    sqlite3 "${1}" "insert into games (player_one, player_two, score_one, score_two, accepted) values ( ${player_one}, ${player_two}, 11, ${loser_score}, true);"
  else
    sqlite3 "${1}" "insert into games (player_one, player_two, score_one, score_two, accepted) values ( ${player_one}, ${player_two}, ${loser_score}, 11, true);"
  fi
done

for (( i = 0; i < $(( maxGames / 10 )); i++ )); do
  player_one=$(( 1 + RANDOM % max ))
  player_two=$(( 1 + RANDOM % max ))

  while (( player_one == player_two )); do
    player_two=$(( RANDOM % max ))
  done

  loser_score=$(( RANDOM % 10 ))

  if (( RANDOM % 2 )); then
    sqlite3 "${1}" "insert into games (player_one, player_two, score_one, score_two, accepted) values ( ${player_one}, ${player_two}, 11, ${loser_score}, false);"
  else
    sqlite3 "${1}" "insert into games (player_one, player_two, score_one, score_two, accepted) values ( ${player_one}, ${player_two}, ${loser_score}, 11, false);"
  fi
done
