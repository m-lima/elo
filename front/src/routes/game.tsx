import { createMemo, createResource, createSignal, For, Show, Suspense } from 'solid-js';
import { Navigator, useNavigate, useParams } from '@solidjs/router';

import { useStore } from '../store';
import { Loading, Main, error } from '../pages';
import { action, icon, prompt } from '../components';
import {
  type Getter,
  type Player as PlayerType,
  type Game as GameType,
  type History as HistoryType,
} from '../types';
import { Maybe, date } from '../util';

import './game.css';

export const Game = () => {
  const params = useParams<{ id: string }>();
  const navigate = useNavigate();

  const store = useStore();
  const players = store.usePlayers();
  const games = store.useGames();

  const game = createMemo(
    () => {
      if (params.id.trim() === '') {
        return;
      }

      const id = Number(params.id);
      if (Number.isNaN(id)) {
        return;
      }

      return games()?.find(g => g.id === id);
    },
    undefined,
    { equals: false },
  );

  const [selectedGame, setSelectedGame] = createSignal<GameType | undefined>();

  const playerOne = createMemo(
    () =>
      Maybe.from(game())
        .map(g => findPlayer(g.playerOne, players()))
        .then(p => {
          return {
            id: p.id,
            name: p.name,
          };
        }),
    undefined,
    { equals: false },
  );

  const playerTwo = createMemo(
    () =>
      Maybe.from(game())
        .map(g => findPlayer(g.playerTwo, players()))
        .then(p => {
          return {
            id: p.id,
            name: p.name,
          };
        }),
    undefined,
    { equals: false },
  );

  const history = createMemo(
    () =>
      Maybe.from(game())
        .map(g => store.getGameHistory(g.id))
        .else(createResource(() => Promise.resolve([]))[0]),
    undefined,
    { equals: false },
  );

  const editGame = (gameTemplate?: GameTemplate) => {
    if (gameTemplate === undefined) {
      return;
    }

    const gameInner = game();
    if (gameInner === undefined) {
      return;
    }

    setSelectedGame({
      ...gameInner,
      playerOne: gameTemplate.playerOne,
      playerTwo: gameTemplate.playerTwo,
      scoreOne: gameTemplate.scoreOne,
      scoreTwo: gameTemplate.scoreTwo,
      challenge: gameTemplate.challenge,
      deleted: gameTemplate.deleted,
      millis: gameTemplate.millis,
    });
  };

  return (
    <Suspense fallback=<Loading />>
      <Show when={game() !== undefined} fallback=<error.NotFound />>
        {Maybe.from(selectedGame()).then(g => (
          <prompt.Edit
            visible={() => true}
            hide={setSelectedGame}
            store={store}
            players={players}
            game={g}
          />
        ))}
        <action.Actions>
          <action.Edit
            text='Edit'
            action={() => {
              editGame(game());
            }}
          />
        </action.Actions>
        <Main>
          <div class='routes-game'>
            <div class='current'>
              <div class='player xlarge'>{playerName(navigate, playerOne())}</div>
              <div class='player xlarge'>{playerName(navigate, playerTwo())}</div>
              <div class='score large'>
                <b>{game()?.scoreOne}</b>
              </div>
              <div class='score large'>
                <b>{game()?.scoreTwo}</b>
              </div>
              <div>{Maybe.from(game()).then(g => rating(g.ratingOne, g.ratingDelta))}</div>
              <div>{Maybe.from(game()).then(g => rating(g.ratingTwo, -g.ratingDelta))}</div>
              <div class='full'>
                {Maybe.from(game()).then(g => date.toLongString(new Date(g.millis)))}
              </div>
              <div
                classList={{
                  marker: true,
                  medium: true,
                  active: game()?.challenge === true,
                }}
              >
                <icon.Swords /> <span>Challenge</span>
              </div>
              <div
                classList={{
                  marker: true,
                  medium: true,
                  active: game()?.deleted === true,
                }}
              >
                <icon.Trash /> <span>Deleted</span>
              </div>
            </div>
            <History
              history={history()}
              game={game}
              players={players}
              editGame={editGame}
              navigate={navigate}
            />
          </div>
        </Main>
      </Show>
    </Suspense>
  );
};

const rating = (rating: number, delta: number) => {
  if (delta > 0) {
    return (
      <span class='positive'>
        {rating.toFixed(2)} <icon.Up /> {(rating + delta).toFixed(2)}
      </span>
    );
  } else if (delta < 0) {
    return (
      <span class='negative'>
        {rating.toFixed(2)} <icon.Down /> {(rating + delta).toFixed(2)}
      </span>
    );
  } else {
    return <>{rating.toFixed(2)}</>;
  }
};

const History = (props: {
  history: Getter<HistoryType[]>;
  game: Getter<GameType>;
  players: Getter<PlayerType[]>;
  editGame: (game: GameTemplate) => void;
  navigate: Navigator;
}) => (
  <Suspense
    fallback=<span>
      <icon.Spinner /> Loading
    </span>
  >
    <For each={props.history()}>
      {(h, i) =>
        historyRow(
          h,
          (props.history()?.length ?? 0) - i(),
          props.players,
          props.editGame,
          props.navigate,
        )
      }
    </For>
  </Suspense>
);

const historyRow = (
  history: HistoryType,
  version: number,
  players: Getter<PlayerType[]>,
  editGame: (game: GameTemplate) => void,
  navigate: Navigator,
) => (
  <>
    <div class='version'>
      <div class='header'>
        <b>Version {version}</b>
        <span class='date wrap'>{date.toLongString(new Date(history.createdMs))}</span>
        <span
          classList={{
            marker: true,
            active: history.challenge,
          }}
        >
          <icon.Swords />
        </span>
        <span
          classList={{
            marker: true,
            active: history.deleted,
          }}
        >
          <icon.Trash />
        </span>
      </div>
      <div class='content'>
        <div class='wrap'>
          {playerName(
            navigate,
            players()?.find(p => p.id === history.playerOne),
          )}{' '}
          <b>{history.scoreOne}</b> <icon.Cancel /> <b>{history.scoreTwo}</b>{' '}
          {playerName(
            navigate,
            players()?.find(p => p.id === history.playerTwo),
          )}{' '}
        </div>
        <span class='wrap'>{date.toLongString(new Date(history.millis))}</span>{' '}
        <button
          class='restore'
          onClick={() => {
            editGame(history);
          }}
        >
          <icon.Restore /> Restore
        </button>
      </div>
    </div>
  </>
);

const findPlayer = (player: number, players?: PlayerType[]) => {
  if (players === undefined) {
    return;
  }

  return players.find(p => p.id === player);
};

const playerName = (navigate: Navigator, player?: Player) => {
  if (player !== undefined) {
    return (
      <a
        class='wrap'
        onClick={evt => {
          evt.stopPropagation();
          navigate(`/player/${player.id}`);
        }}
      >
        {player.name}
      </a>
    );
  } else {
    return <span class='unknown'>{'<unknown>'}</span>;
  }
};

type Player = {
  id: number;
  name: string;
};

type GameTemplate = Pick<
  GameType,
  'playerOne' | 'playerTwo' | 'scoreOne' | 'scoreTwo' | 'challenge' | 'deleted' | 'millis'
>;
