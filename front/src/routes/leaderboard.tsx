import { Accessor, For, JSX, Setter, Suspense, createMemo, createSignal } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { Loading, Main } from '../pages';
import { action, prompt, icon } from '../components';
import { type EnrichedPlayer, type User } from '../types';
import { useStore } from '../store';

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

// TODO: Make this more responsive
export const Leaderboard = () => {
  const store = useStore();
  const players = store.useEnrichedPlayers();
  const games = store.useGames();
  const self = store.useSelf();
  const [promptVisible, setPromptVisible] = createSignal(false);
  const navigate = useNavigate();
  const [sortPivot, setSortPivot] = createSignal<Pivot>('position');
  const [sortDescending, setSortDescending] = createSignal(true);

  const sortedPlayers = createMemo(
    () => {
      const pivot = sortPivot();
      const descending = sortDescending();

      // TODO: Test if this needs to be copied
      return players().sort((a, b) => {
        if (pivot === 'name') {
          const pivotA = a.name;
          const pivotB = b.name;

          if (sortDescending()) {
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
    [],
    { equals: false },
  );

  return (
    <Suspense fallback=<Loading />>
      <prompt.Game
        visible={promptVisible}
        hide={() => setPromptVisible(false)}
        store={store}
        self={() => players().find(p => p.id === self()?.id)}
        players={players}
        games={games}
      />
      <action.Actions>
        <action.Game action={() => setPromptVisible(true)} />
      </action.Actions>
      <Main>
        <table class='clickable'>
          <thead class='routes-leaderboard-table-header'>
            <tr>
              <th />
              {header('#', 'position', sortPivot, setSortPivot, sortDescending, setSortDescending)}
              {header('Player', 'name', sortPivot, setSortPivot, sortDescending, setSortDescending)}
              {header(
                'Rating',
                'rating',
                sortPivot,
                setSortPivot,
                sortDescending,
                setSortDescending,
              )}
              {header('Games', 'games', sortPivot, setSortPivot, sortDescending, setSortDescending)}
              {header('Wins', 'wins', sortPivot, setSortPivot, sortDescending, setSortDescending)}
              {header(
                'Losses',
                'losses',
                sortPivot,
                setSortPivot,
                sortDescending,
                setSortDescending,
              )}
              {header(
                'Challenges won',
                'challengesWon',
                sortPivot,
                setSortPivot,
                sortDescending,
                setSortDescending,
              )}
              {header(
                'Challenges lost',
                'challengesLost',
                sortPivot,
                setSortPivot,
                sortDescending,
                setSortDescending,
              )}
              {header(
                'Points won',
                'pointsWon',
                sortPivot,
                setSortPivot,
                sortDescending,
                setSortDescending,
              )}
              {header(
                'Points lost',
                'pointsLost',
                sortPivot,
                setSortPivot,
                sortDescending,
                setSortDescending,
              )}
            </tr>
          </thead>
          <tbody>
            <For each={sortedPlayers()}>
              {p => playerRow(p, navigate, getIcon(p.position, players().length), self())}
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
  sortPivot: Accessor<Pivot>,
  setSortPivot: Setter<Pivot>,
  sortDescending: Accessor<boolean>,
  setSortDescending: Setter<boolean>,
) => (
  <th
    onClick={() => {
      sortPivot() === field ? setSortDescending(d => !d) : setSortPivot(() => field);
    }}
  >
    {name}
    <span class='routes-leaderboard-sort'>{sortIcon(sortPivot(), field, sortDescending())}</span>
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
