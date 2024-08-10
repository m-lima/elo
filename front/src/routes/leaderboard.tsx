import { For, JSX, Suspense, createMemo, createSignal } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { Loading, Main } from '../pages';
import { action, prompt, icon } from '../components';
import { type EnrichedPlayer, type User } from '../types';
import { useStore } from '../store';

import './leaderboard.css';

// TODO: Make this more responsive
export const Leaderboard = () => {
  const store = useStore();
  const players = store.useEnrichedPlayers();
  const self = store.useSelf();
  const [promptVisible, setPromptVisible] = createSignal(false);
  const [sortPivot, setSortPivot] = createSignal<Pivot>('position');
  const [sortDescending, setSortDescending] = createSignal(true);
  const navigate = useNavigate();

  const sortedPlayers = createMemo(
    () => {
      const pivot = sortPivot();
      const descending = sortDescending();

      return players()
        .filter(p => p.games > 0)
        .map(p => {
          return {
            id: p.id,
            position: p.position,
            name: p.name,
            rating: p.rating,
            games: p.games,
            wins: p.wins,
            losses: p.losses,
            challengesWon: p.challengesWon,
            challengesLost: p.challengesLost,
            pointsWon: p.pointsWon,
            pointsLost: p.pointsLost,
          };
        })
        .sort((a, b) => {
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

  const header = (name: string, field: Pivot) => (
    <th
      onClick={() => {
        sortPivot() === field ? setSortDescending(d => !d) : setSortPivot(() => field);
      }}
    >
      {name}
      <span class='routes-leaderboard-sort'>{sortIcon(sortPivot(), field, sortDescending())}</span>
    </th>
  );

  return (
    <Suspense fallback=<Loading />>
      <prompt.Register
        visible={promptVisible}
        hide={() => setPromptVisible(false)}
        store={store}
        players={players}
        self={() => self()?.id}
      />
      <action.Actions>
        <action.Game action={() => setPromptVisible(true)} />
      </action.Actions>
      <Main>
        <div class='routes-leaderboard'>
          <div class='routes-leaderboard-table'>
            <table class='clickable'>
              <thead class='routes-leaderboard-table-header'>
                <tr>
                  <th />
                  {header('#', 'position')}
                  {header('Player', 'name')}
                  {header('Rating', 'rating')}
                  {header('Games', 'games')}
                  {header('Wins', 'wins')}
                  {header('Losses', 'losses')}
                  {header('Challenges won', 'challengesWon')}
                  {header('Challenges lost', 'challengesLost')}
                  {header('Points won', 'pointsWon')}
                  {header('Points lost', 'pointsLost')}
                </tr>
              </thead>
              <tbody>
                <For each={sortedPlayers()}>
                  {p => playerRow(p, navigate, getIcon(p.position, sortedPlayers().length), self())}
                </For>
              </tbody>
            </table>
          </div>
        </div>
      </Main>
    </Suspense>
  );
};

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
  player: LeaderboardPlayer,
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

type LeaderboardPlayer = Pick<
  EnrichedPlayer,
  | 'id'
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

type Pivot = keyof Omit<LeaderboardPlayer, 'id'>;
