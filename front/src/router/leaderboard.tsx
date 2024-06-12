import { For, JSX, Suspense } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { usePlayers } from '../store';
import { Player, byPosition } from '../types';
import { icon, Loading } from '../components';

import './leaderboard.css';

export const Leaderboard = () => {
  const players = usePlayers();
  return <Suspense fallback={<Loading />}>{playerTable(players())}</Suspense>;
};

const playerTable = (players: Player[] = []) => {
  const navigate = useNavigate();

  const getIcon = (i: number, l: number) => {
    switch (i) {
      case 0:
        return (
          <span class='router-leaderboard-first'>
            <icon.Crown />
          </span>
        );
      case 1:
        return (
          <span class='router-leaderboard-second'>
            <icon.Medal />
          </span>
        );
      case 2:
        return (
          <span class='router-leaderboard-third'>
            <icon.Certificate />
          </span>
        );
      case l - 4:
        return <icon.Mosquito />;
      case l - 3:
        return <icon.Poop />;
      case l - 2:
        return <icon.Worm />;
      case l - 1:
        return <icon.Skull />;
    }
    return;
  };

  players.sort(byPosition);

  return (
    <table class='clickable'>
      <thead>
        <tr>
          <th />
          <th>#</th>
          <th>Player</th>
          <th>Score</th>
        </tr>
      </thead>
      <tbody>
        <For each={players}>{(p, i) => playerRow(navigate, p, getIcon(i(), players.length))}</For>
      </tbody>
    </table>
  );
};

const playerRow = (navigate: Navigator, player: Player, icon?: JSX.Element) => {
  return (
    <tr
      onClick={() => {
        navigate(`/player/${player.id}`);
      }}
    >
      <td class='router-leaderboard-icon'>{icon}</td>
      <td>{player.position}</td>
      <td>{player.name}</td>
      <td>{player.score}</td>
    </tr>
  );
};
