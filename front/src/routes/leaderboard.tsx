import { For, JSX, Suspense } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { Loading, Main } from '../pages';
import { Action, Actions, icon } from '../components';
import { type User, type Player } from '../types';
import { useStore } from '../store';

import './leaderboard.css';

export const Leaderboard = () => {
  const store = useStore();
  const players = store.getPlayers();
  const self = store.getSelf();
  const navigate = useNavigate();

  return (
    <Suspense fallback=<Loading />>
      <>
        <Actions>
          <Action
            icon=<icon.Add />
            text='New game'
            action={() => {
              console.debug('Clicked');
            }}
          />
        </Actions>
        <Main>
          <table class='clickable'>
            <thead>
              <tr>
                <th />
                <th>#</th>
                <th>Player</th>
                <th>Rating</th>
                <th>Games</th>
                <th>Wins</th>
                <th>Losses</th>
                <th>Points won</th>
                <th>Points lost</th>
              </tr>
            </thead>
            <tbody>
              <For each={players()}>
                {(p, i) => playerRow(p, i, navigate, getIcon(i(), players()?.length), self())}
              </For>
            </tbody>
          </table>
        </Main>
      </>
    </Suspense>
  );
};

const playerRow = (
  player: Player,
  position: () => number,
  navigate: Navigator,
  icon?: JSX.Element,
  self?: User,
) => {
  return (
    <tr
      class={self?.id === player.id ? 'routes-leaderboard-self' : undefined}
      onClick={() => {
        navigate(`/player/${player.id}`);
      }}
    >
      <td class='routes-leaderboard-icon'>{icon}</td>
      <td>{position() + 1}</td>
      <td>{player.name}</td>
      <td>{player.rating.toFixed(2)}</td>
      <td>{player.wins + player.losses}</td>
      <td>{player.wins}</td>
      <td>{player.losses}</td>
      <td>{player.pointsWon}</td>
      <td>{player.pointsLost}</td>
    </tr>
  );
};

const getIcon = (position: number, length: number = NaN) => {
  switch (position) {
    case 0:
      return (
        <span class='routes-leaderboard-first'>
          <icon.Crown />
        </span>
      );
    case 1:
      return (
        <span class='routes-leaderboard-second'>
          <icon.Medal />
        </span>
      );
    case 2:
      return (
        <span class='routes-leaderboard-third'>
          <icon.Certificate />
        </span>
      );
    case length - 4:
      return <icon.Mosquito />;
    case length - 3:
      return <icon.Poop />;
    case length - 2:
      return <icon.Worm />;
    case length - 1:
      return <icon.Skull />;
  }
  return;
};
