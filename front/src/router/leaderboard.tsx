import { For, JSX, Suspense } from 'solid-js';

import { usePlayers } from '../store';
import { Player, byPosition } from '../types';
import { icon, Loading } from '../components';

import 'leaderboard.css';

type PreparedPlayer = Player & {
  icon?: () => JSX.Element;
  class?: string;
};

const PlayerTable = (props: { players: Player[] }) => {
  const length = props.players.length;
  const preparedPlayers: PreparedPlayer[] = props.players.sort(byPosition).map((p, i) => {
    if (i === 0) {
      return { icon: icon.Crown, class: 'router-leaderboard-first', ...p };
    }
    if (i === 1) {
      return { icon: icon.Medal, class: 'router-leaderboard-second', ...p };
    }
    if (i === 2) {
      return { icon: icon.Certificate, class: 'router-leaderboard-third', ...p };
    }
    if (i === length - 4) {
      return { icon: icon.Mosquito, ...p };
    }
    if (i === length - 3) {
      return { icon: icon.Poop, ...p };
    }
    if (i === length - 2) {
      return { icon: icon.Worm, ...p };
    }
    if (i === length - 1) {
      return { icon: icon.Skull, ...p };
    }
    return p;
  });

  return (
    <div class='router-leaderboard'>
      <table>
        <thead>
          <tr>
            <th scope='col' />
            <th scope='col'>#</th>
            <th scope='col'>Player</th>
            <th scope='col'>Score</th>
          </tr>
        </thead>
        <tbody>
          <For each={preparedPlayers}>
            {p => (
              <tr class={p.class}>
                <td class={p.class}>{p.icon !== undefined ? p.icon() : ''}</td>
                <td>{p.position}</td>
                <td>{p.name}</td>
                <td>{p.score}</td>
              </tr>
            )}
          </For>
        </tbody>
      </table>
    </div>
  );
};

export const Leaderboard = () => {
  const players = usePlayers();

  return (
    <Suspense fallback={<Loading />}>
      <PlayerTable players={players() || []} />
    </Suspense>
  );
};
