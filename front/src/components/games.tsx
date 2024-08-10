import { createSignal, createMemo, For, Show } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { icon } from '.';
import { type Getter, type EnrichedGame } from '../types';
import * as util from '../util';
import * as consts from '../consts';

import './games.css';

export const Games = (props: { games: Getter<EnrichedGame[]> }) => {
  const [limit, setLimit] = createSignal(consts.limit.gameList);
  const navigate = useNavigate();

  const filteredGames = createMemo(() => props.games()?.filter((_, i) => i < limit()), [], {
    equals: false,
  });

  return (
    <div class='components-games'>
      <table class='components-games clickable'>
        <tbody>
          <For each={filteredGames()}>{g => gameRow(g, navigate)}</For>
        </tbody>
      </table>
      <Show when={Number(props.games()?.length) > limit()}>
        <div class='more' onClick={() => setLimit(l => l + consts.limit.gameList)}>
          <icon.DoubleDown />
        </div>
      </Show>
    </div>
  );
};

const gameRow = (game: EnrichedGame, navigate: Navigator) => (
  <tr
    class={game.deleted ? 'deleted' : undefined}
    onClick={() => {
      navigate(`/game/${game.id}`);
    }}
  >
    <td class='right'>
      {playerName(evt => {
        evt.stopPropagation();
        navigate(`/player/${game.playerOne}`);
      }, game.playerOneName)}
    </td>
    <td class='right'>{game.scoreOne}</td>
    {game.challenge ? (
      <td class='challenge'>
        <icon.Swords />
      </td>
    ) : (
      <td class='versus'>
        <icon.Cancel />
      </td>
    )}
    <td>{game.scoreTwo}</td>
    <td>
      {playerName(evt => {
        evt.stopPropagation();
        navigate(`/player/${game.playerTwo}`);
      }, game.playerTwoName)}
    </td>
    <td class='tail'>
      {playerRating(game.ratingDelta)}
      <span class='right date'>{util.date.toString(new Date(game.millis))}</span>
    </td>
  </tr>
);

const playerName = (navigate: (evt: MouseEvent) => void, name?: string) => {
  if (name !== undefined) {
    return <a onClick={navigate}>{name}</a>;
  } else {
    return <span class='unknown'>{'<unknown>'}</span>;
  }
};

const playerRating = (rating?: number) => {
  if (rating !== undefined) {
    if (rating > 0) {
      return <span class='positive'>+{rating.toFixed(2)}</span>;
    } else if (rating < 0) {
      return <span class='negative'>{rating.toFixed(2)}</span>;
    } else {
      return <span>{rating.toFixed(2)}</span>;
    }
  } else {
    return <></>;
  }
};
