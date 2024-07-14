import { createSignal, createMemo, For, Show } from 'solid-js';
import { A } from '@solidjs/router';

import { icon } from '.';
import { type Getter, type EnrichedGame } from '../types';
import { monthToString } from '../util';

import './games.css';

export const Games = (props: { games: Getter<EnrichedGame[]> }) => {
  const [limit, setLimit] = createSignal(100);

  const filteredGames = createMemo(() => props.games()?.filter((_, i) => i < limit()), [], {
    equals: false,
  });

  return (
    <div class='components-games'>
      <table class='components-games'>
        <tbody>
          <For each={filteredGames()}>{gameRow}</For>
        </tbody>
      </table>
      <Show when={Number(props.games()?.length) > limit()}>
        <div class='components-games-more' onClick={() => setLimit(l => l + 100)}>
          <icon.DoubleDown />
        </div>
      </Show>
    </div>
  );
};

const gameRow = (game: EnrichedGame) => {
  return (
    <tr>
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
      <td class='components-games-tail'>
        {playerRating(game.ratingDelta)}
        <span class='components-games-align-right components-games-date'>
          {dateToString(new Date(game.createdMs))}
        </span>
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

const dateToString = (date: Date) =>
  `${String(date.getDate()).padStart(2, '0')}-${monthToString(date.getMonth())} ${String(date.getHours()).padStart(2, '0')}:${String(date.getMinutes()).padStart(2, '0')}`;
