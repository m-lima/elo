import { For, JSX, Suspense } from 'solid-js';

import { usePlayers } from '../store';
import { Player, byPosition } from '../types';
import { icon, Loading } from '../components';

import './leaderboard.css';

export const Leaderboard = () => {
  const players = usePlayers();

  console.log('Leaderboard', players);

  return <Suspense fallback={<Loading />}>{playerTable(players())}</Suspense>;
};

const playerTable = (players: Player[] = []) => {
  console.log('Table', players);

  const getIcon = (i: number, l: number) => {
    switch (i) {
      case 0:
        return (
          <td class='router-leaderboard-icon' id='first'>
            <icon.Crown />
          </td>
        );
      case 1:
        return (
          <td class='router-leaderboard-icon' id='second'>
            <icon.Medal />
          </td>
        );
      case 2:
        return (
          <td class='router-leaderboard-icon' id='third'>
            <icon.Certificate />
          </td>
        );
      case l - 4:
        return (
          <td class='router-leaderboard-icon'>
            <icon.Mosquito />
          </td>
        );
      case l - 3:
        return (
          <td class='router-leaderboard-icon'>
            <icon.Poop />
          </td>
        );
      case l - 2:
        return (
          <td class='router-leaderboard-icon'>
            <icon.Worm />
          </td>
        );
      case l - 1:
        return (
          <td class='router-leaderboard-icon'>
            <icon.Skull />
          </td>
        );
    }
    return <td class='router-leaderboard-icon' />;
  };

  players.sort(byPosition);

  return (
    <div class='router-leaderboard'>
      <h1>Leaderboard</h1>
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
          <For each={players}>{(p, i) => playerRow(p, getIcon(i(), players.length))}</For>
        </tbody>
      </table>
    </div>
  );
};

const playerRow = (player: Player, icon?: JSX.Element) => {
  return (
    <tr>
      <td class='router-leaderboard-icon'>{icon}</td>
      <td>{player.position}</td>
      <td>{player.name}</td>
      <td>{player.score}</td>
    </tr>
  );
};
