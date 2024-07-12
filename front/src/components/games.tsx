import { For, createMemo } from 'solid-js';
import { A } from '@solidjs/router';

import { icon } from '.';
import { type Getter, type Game, type Player } from '../types';
import { monthToString } from '../util';

import './games.css';

// TODO: Make this more responsive
export const Games = (props: {
  games: Getter<Game[]>;
  players: Getter<Player[]>;
  player?: Getter<number>;
}) => {
  const games = createMemo(() => parseGames(props.games(), props.players(), props.player?.()));

  return (
    <table>
      <tbody>
        <For each={games()}>{gameRow}</For>
      </tbody>
    </table>
  );
};

const gameRow = (game: ParsedGame) => {
  return (
    <tr>
      <td class='components-games-align-right'>{playerRating(game.ratingOneDelta)}</td>
      <td class='components-games-align-right'>{playerName(game.playerOne, game.playerOneName)}</td>
      <td class='components-games-align-right'>{game.scoreOne}</td>
      {game.challenge ? (
        <td class='components-games-align-challenge'>
          <icon.Swords />
        </td>
      ) : (
        <td class='components-games-align-versus'>
          <icon.Cancel />
        </td>
      )}
      <td>{game.scoreTwo}</td>
      <td>{playerName(game.playerTwo, game.playerTwoName)}</td>
      <td>{playerRating(game.ratingTwoDelta)}</td>
      <td class='components-games-align-right components-games-date'>
        {dateToString(game.created)}
      </td>
    </tr>
  );
};

const playerName = (id: number, name?: string) => {
  if (name !== undefined) {
    return <A href={`/player/${id}`}>{name}</A>;
  } else {
    return <span class='components-games-unknown'>{'<unknown>'}</span>;
  }
};

const playerRating = (rating?: number) => {
  if (rating !== undefined) {
    if (rating > 0) {
      return <span class='components-games-rating-positive'>+{rating.toFixed(2)}</span>;
    } else if (rating < 0) {
      return <span class='components-games-rating-negative'>{rating.toFixed(2)}</span>;
    } else {
      return <span>{rating.toFixed(2)}</span>;
    }
  } else {
    return <></>;
  }
};

type ParsedGame = Game & {
  readonly playerOneName?: string;
  readonly playerTwoName?: string;
  readonly ratingOneDelta?: number;
  readonly ratingTwoDelta?: number;
  readonly created: Date;
};

const parseGames = (games: Game[] = [], players: Player[] = [], player?: number): ParsedGame[] => {
  const playerRatings = new Map(players.map(p => [p.id, { name: p.name, rating: p.rating }]));

  const parsedGames = games.map(g => {
    const playerOne = playerRatings.get(g.playerOne);
    const playerTwo = playerRatings.get(g.playerTwo);
    let ratingOneDelta: number | undefined;
    let ratingTwoDelta: number | undefined;

    if (playerOne !== undefined) {
      ratingOneDelta = playerOne.rating - g.ratingOne;
      playerOne.rating = g.ratingOne;
    }

    if (playerTwo !== undefined) {
      ratingTwoDelta = playerTwo.rating - g.ratingTwo;
      playerTwo.rating = g.ratingTwo;
    }

    return {
      ...g,
      playerOneName: playerOne?.name,
      playerTwoName: playerTwo?.name,
      ratingOneDelta,
      ratingTwoDelta,
      created: new Date(g.createdMs),
    };
  });

  if (player !== undefined) {
    return parsedGames.filter(g => g.playerOne === player || g.playerTwo === player);
  } else {
    return parsedGames;
  }
};

const dateToString = (date: Date) =>
  `${String(date.getDate()).padStart(2, '0')}/${monthToString(date.getMonth())} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
