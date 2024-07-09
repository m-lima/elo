import { For, JSX, Signal, Suspense, createMemo, createSignal } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { Loading, Main } from '../pages';
import { action, prompt, icon } from '../components';
import { type User } from '../types';
import { useStore } from '../store';
import { type EnrichedPlayer, enrichPlayers } from '../util';

import './leaderboard.css';

type Pivot = keyof Pick<
  EnrichedPlayer,
  | 'position'
  | 'name'
  | 'rating'
  | 'games'
  | 'wins'
  | 'losses'
  | 'challengesWon'
  | 'challengesLost'
  | 'pointsWon'
  | 'pointsLost'
>;

export const Leaderboard = () => {
  const store = useStore();
  const players = store.getPlayers();
  const games = store.getGames();
  const self = store.getSelf();
  const [promptVisible, setPromptVisible] = createSignal(false);
  const navigate = useNavigate();
  const sortPivot = createSignal<Pivot>('position');
  const sortDescending = createSignal(true);

  const enrichedPlayers = createMemo(() => enrichPlayers(players(), games()));
  const sortedPlayers = createMemo(
    () => {
      const pivot = sortPivot[0]();
      const descending = sortDescending[0]();

      return enrichedPlayers().sort((a, b) => {
        if (pivot === 'name') {
          const pivotA = a.name;
          const pivotB = b.name;

          if (sortDescending[0]()) {
            return pivotA.localeCompare(pivotB);
          } else {
            return pivotB.localeCompare(pivotA);
          }
        } else {
          const pivotA = a[pivot];
          const pivotB = b[pivot];

          if (descending) {
            return pivotA - pivotB;
          } else {
            return pivotB - pivotA;
          }
        }
      });
    },
    enrichedPlayers(),
    { equals: false },
  );

  return (
    <Suspense fallback=<Loading />>
      <prompt.Game
        visible={promptVisible}
        hide={() => setPromptVisible(false)}
        store={store}
        self={() => players()?.find(p => p.id === self()?.id)}
        players={players}
        games={games}
      />
      <action.Actions>
        <action.Game action={() => setPromptVisible(true)} />
      </action.Actions>
      <Main>
        <table class='clickable'>
          <thead>
            <tr>
              <th />
              {header('#', 'position', sortPivot, sortDescending)}
              {header('Player', 'name', sortPivot, sortDescending)}
              {header('Rating', 'rating', sortPivot, sortDescending)}
              {header('Games', 'games', sortPivot, sortDescending)}
              {header('Wins', 'wins', sortPivot, sortDescending)}
              {header('Losses', 'losses', sortPivot, sortDescending)}
              {header('Challenges won', 'challengesWon', sortPivot, sortDescending)}
              {header('Challenges lost', 'challengesLost', sortPivot, sortDescending)}
              {header('Points won', 'pointsWon', sortPivot, sortDescending)}
              {header('Points lost', 'pointsLost', sortPivot, sortDescending)}
            </tr>
          </thead>
          <tbody>
            <For each={sortedPlayers()}>
              {p => playerRow(p, navigate, getIcon(p.position, players()?.length), self())}
            </For>
          </tbody>
        </table>
      </Main>
    </Suspense>
  );
};

const header = (
  name: string,
  field: Pivot,
  [sortPivot, setSortPivot]: Signal<Pivot>,
  [sortDescending, setSortDescending]: Signal<boolean>,
) => (
  <th
    onClick={() => {
      sortPivot() === field ? setSortDescending(d => !d) : setSortPivot(() => field);
    }}
  >
    {name} {sortIcon(sortPivot(), field, sortDescending())}
  </th>
);

const sortIcon = (pivot: Pivot, name: Pivot, descending: boolean) => {
  if (pivot === name) {
    if (descending) {
      return <icon.SortUp />;
    } else {
      return <icon.SortDown />;
    }
  } else {
    return <icon.SortBoth />;
  }
};

const playerRow = (
  player: EnrichedPlayer,
  navigate: Navigator,
  badge?: JSX.Element,
  self?: User,
) => {
  return (
    <tr
      class={self?.id === player.id ? 'routes-leaderboard-self' : undefined}
      onClick={() => {
        navigate(`/player/${player.id}`);
      }}
    >
      <td class='routes-leaderboard-badge'>{badge}</td>
      <td>{player.position}</td>
      <td>{player.name}</td>
      <td>{player.rating.toFixed(2)}</td>
      <td>{player.games}</td>
      <td>{player.wins}</td>
      <td>{player.losses}</td>
      <td>{player.challengesWon}</td>
      <td>{player.challengesLost}</td>
      <td>{player.pointsWon}</td>
      <td>{player.pointsLost}</td>
    </tr>
  );
};

const getIcon = (position: number, length: number = NaN) => {
  switch (position) {
    case 1:
      return (
        <span class='routes-leaderboard-first'>
          <icon.Crown />
        </span>
      );
    case 2:
      return (
        <span class='routes-leaderboard-second'>
          <icon.Medal />
        </span>
      );
    case 3:
      return (
        <span class='routes-leaderboard-third'>
          <icon.Certificate />
        </span>
      );
    case length - 3:
      return <icon.Mosquito />;
    case length - 2:
      return <icon.Poop />;
    case length - 1:
      return <icon.Worm />;
    case length - 0:
      return <icon.Skull />;
  }
  return;
};
