import { For, JSX, Suspense, createMemo, createSignal } from 'solid-js';
import { Navigator, useNavigate } from '@solidjs/router';

import { Loading, Main } from '../pages';
import { action, prompt, icon } from '../components';
import { type User } from '../types';
import { useStore } from '../store';
import { type EnrichedPlayer, enrichPlayers } from '../util';

import './leaderboard.css';

export const Leaderboard = () => {
  const store = useStore();
  const players = store.getPlayers();
  const games = store.getGames();
  const self = store.getSelf();
  const [promptVisible, setPromptVisible] = createSignal(false);
  const navigate = useNavigate();

  const enrichedPlayers = createMemo(() => enrichPlayers(players(), games()));

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
              <th>#</th>
              <th>Player</th>
              <th>Rating</th>
              <th>Games</th>
              <th>Wins</th>
              <th>Losses</th>
              <th>Challenges won</th>
              <th>Challenges lost</th>
              <th>Points won</th>
              <th>Points lost</th>
            </tr>
          </thead>
          <tbody>
            <For each={enrichedPlayers()}>
              {(p, i) => playerRow(p, i, navigate, getIcon(i(), players()?.length), self())}
            </For>
          </tbody>
        </table>
      </Main>
    </Suspense>
  );
};

const playerRow = (
  player: EnrichedPlayer,
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
