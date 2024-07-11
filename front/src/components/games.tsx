import { For, createMemo } from 'solid-js';
import { A } from '@solidjs/router';

import { icon } from '.';
import { type Game, type Player } from '../types';
import { type Getter, monthToString } from '../util';

import './games.css';

// TODO: Make this more responsive
// TODO: FIXME: The rating is broken
export const Games = (props: {
  games: Getter<Game[]>;
  players: Getter<Player[]>;
  player?: Getter<number>;
}) => {
  const games = createMemo(() =>
    parseGames(props.games(), props.players(), props.player?.()).reverse(),
  );

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
      <td class='components-games-align-right'>{playerRating(game.ratingOne)}</td>
      <td class='components-games-align-right'>{playerName(game.playerOne)}</td>
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
      <td>{playerName(game.playerTwo)}</td>
      <td>{playerRating(game.ratingTwo)}</td>
      <td class='components-games-align-right components-games-date'>
        {dateToString(game.created)}
      </td>
    </tr>
  );
};

const playerName = (player?: Player) => {
  if (player !== undefined) {
    return <A href={`/player/${player.id}`}>{player.name}</A>;
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

type ParsedGame = {
  readonly id: number;
  readonly playerOne?: Player;
  readonly playerTwo?: Player;
  readonly scoreOne: number;
  readonly scoreTwo: number;
  readonly ratingOne?: number;
  readonly ratingTwo?: number;
  readonly challenge: boolean;
  readonly created: Date;
};

const parseGames = (games: Game[] = [], players: Player[] = [], player?: number): ParsedGame[] => {
  const playerRating: Map<number, number> = new Map();

  const parsedGames = games.map(g => {
    const playerOne = players.find(p => p.id === g.playerOne);
    const playerTwo = players.find(p => p.id === g.playerTwo);
    let ratingOne: number | undefined;
    let ratingTwo: number | undefined;

    if (playerOne !== undefined) {
      const maybeRating = playerRating.get(playerOne.id);
      const rating = maybeRating !== undefined ? maybeRating : playerOne.rating;

      ratingOne = rating - g.ratingOne;
      playerRating.set(playerOne.id, g.ratingOne);
    }

    if (playerTwo !== undefined) {
      const maybeRating = playerRating.get(playerTwo.id);
      const rating = maybeRating !== undefined ? maybeRating : playerTwo.rating;

      ratingTwo = rating - g.ratingTwo;
      playerRating.set(playerTwo.id, g.ratingTwo);
    }

    return {
      id: g.id,
      playerOne,
      playerTwo,
      scoreOne: g.scoreOne,
      scoreTwo: g.scoreTwo,
      ratingOne,
      ratingTwo,
      challenge: g.challenge,
      created: new Date(g.createdMs),
    };
  });

  if (player !== undefined) {
    return parsedGames.filter(g => g.playerOne?.id === player || g.playerTwo?.id === player);
  } else {
    return parsedGames;
  }
};

const dateToString = (date: Date) =>
  `${String(date.getDate()).padStart(2, '0')}/${monthToString(date.getMonth())} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
// import { For, createMemo } from 'solid-js';
// import { A } from '@solidjs/router';
//
// import { icon } from '.';
// import { type Game, type Player } from '../types';
// import { type Getter, monthToString } from '../util';
//
// import './games.css';
//
// // TODO: Make this more responsive
// // TODO: FIXME: The rating is broken
// export const Games = (props: {
//   games: Getter<Game[]>;
//   players: Getter<Player[]>;
//   player?: Getter<number>;
// }) => {
//   const games = createMemo(() =>
//     parseGames(props.games(), props.players(), props.player?.()).reverse(),
//   );
//
//   return (
//     <table>
//       <tbody>
//         <For each={games()}>{gameRow}</For>
//       </tbody>
//     </table>
//   );
// };
//
// const gameRow = (game: ParsedGame) => {
//   return (
//     <tr>
//       <td class='components-games-align-right'>{playerRating(game.ratingOne)}</td>
//       <td class='components-games-align-right'>{playerName(game.playerOne)}</td>
//       <td class='components-games-align-right'>{game.scoreOne}</td>
//       {game.challenge ? (
//         <td class='components-games-align-challenge'>
//           <icon.Swords />
//         </td>
//       ) : (
//         <td class='components-games-align-versus'>
//           <icon.Cancel />
//         </td>
//       )}
//       <td>{game.scoreTwo}</td>
//       <td>{playerName(game.playerTwo)}</td>
//       <td>{playerRating(game.ratingTwo)}</td>
//       <td class='components-games-align-right components-games-date'>
//         {dateToString(game.created)}
//       </td>
//     </tr>
//   );
// };
//
// const playerName = (player?: Player) => {
//   if (player !== undefined) {
//     return <A href={`/player/${player.id}`}>{player.name}</A>;
//   } else {
//     return <span class='components-games-unknown'>{'<unknown>'}</span>;
//   }
// };
//
// const playerRating = (rating?: number) => {
//   if (rating !== undefined) {
//     if (rating > 0) {
//       return <span class='components-games-rating-positive'>+{rating.toFixed(2)}</span>;
//     } else if (rating < 0) {
//       return <span class='components-games-rating-negative'>{rating.toFixed(2)}</span>;
//     } else {
//       return <span>{rating.toFixed(2)}</span>;
//     }
//   } else {
//     return <></>;
//   }
// };
//
// type ParsedGame = {
//   readonly id: number;
//   readonly playerOne?: Player;
//   readonly playerTwo?: Player;
//   readonly scoreOne: number;
//   readonly scoreTwo: number;
//   readonly ratingOne?: number;
//   readonly ratingTwo?: number;
//   readonly challenge: boolean;
//   readonly created: Date;
// };
//
// const parseGames = (games: Game[] = [], players: Player[] = [], player?: number): ParsedGame[] => {
//   const playerRatings: Map<number, number> = new Map(players.map(p => [p.id, p.rating]));
//
//   const parsedGames = games.map(g => {
//     const playerOne = playerRatings.get(g.playerOne);
//     const playerTwo = playerRatings.get(g.playerTwo);
//
//     const ratingOne = playerOne !== undefined ? playerOne - g.ratingOne : g.ratingOne;
//     const ratingTwo = playerTwo !== undefined ? playerTwo - g.ratingTwo : g.ratingTwo;
//
//     playerRatings.set(g.playerOne, ratingOne);
//     playerRatings.set(g.playerTwo, ratingTwo);
//
//     // const playerOne = players.find(p => p.id === g.playerOne);
//     // const playerTwo = players.find(p => p.id === g.playerTwo);
//     // let ratingOne: number | undefined;
//     // let ratingTwo: number | undefined;
//     //
//     // if (playerOne !== undefined) {
//     //   const maybeRating = playerRating.get(playerOne.id);
//     //   const rating = maybeRating !== undefined ? maybeRating : playerOne.rating;
//     //
//     //   ratingOne = rating - g.ratingOne;
//     //   playerRating.set(playerOne.id, g.ratingOne);
//     // }
//     //
//     // if (playerTwo !== undefined) {
//     //   const maybeRating = playerRating.get(playerTwo.id);
//     //   const rating = maybeRating !== undefined ? maybeRating : playerTwo.rating;
//     //
//     //   ratingTwo = rating - g.ratingTwo;
//     //   playerRating.set(playerTwo.id, g.ratingTwo);
//     // }
//
//     return {
//       playerOne,
//       playerTwo,
//       ratingOne,
//       ratingTwo,
//       challenge: g.challenge,
//       created: new Date(g.createdMs),
//     };
//   });
//
//   if (player !== undefined) {
//     return parsedGames.filter(g => g.playerOne?.id === player || g.playerTwo?.id === player);
//   } else {
//     return parsedGames;
//   }
// };
//
// const dateToString = (date: Date) =>
//   `${String(date.getDate()).padStart(2, '0')}/${monthToString(date.getMonth())} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
