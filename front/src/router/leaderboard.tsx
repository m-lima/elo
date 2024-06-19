import { For, JSX, Suspense } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { usePlayers } from '../store';
import { type Player } from '../types';
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

  return (
    <table class='clickable'>
      <thead>
        <tr>
          <th />
          <th>#</th>
          <th>Player</th>
          <th>Rating</th>
          <th>Wins</th>
          <th>Losses</th>
          <th>Points won</th>
          <th>Points lost</th>
        </tr>
      </thead>
      <tbody>
        <For each={players}>
          {(p, i) => playerRow(i() + 1, navigate, p, getIcon(i(), players.length))}
        </For>
      </tbody>
    </table>
  );
};

const playerRow = (position: number, navigate: Navigator, player: Player, icon?: JSX.Element) => {
  return (
    <tr
      onClick={() => {
        navigate(`/player/${player.id}`);
      }}
    >
      <td class='router-leaderboard-icon'>{icon}</td>
      <td>{position}</td>
      <td>{player.name}</td>
      <td>{player.rating.toFixed(2)}</td>
      <td>{player.wins}</td>
      <td>{player.losses}</td>
      <td>{player.pointsWon}</td>
      <td>{player.pointsLost}</td>
    </tr>
  );
};
