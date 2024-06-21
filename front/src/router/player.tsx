import { Match, Show, Suspense, Switch, createSignal } from 'solid-js';
import { Navigator, useNavigate, useParams } from '@solidjs/router';

import { useGames, usePlayers, useSelf, useStore } from '../store';
import { type Game, type Player as PlayerType } from '../types';
import { icon, Loading, Games } from '../components';

import './player.css';

export const Player = () => {
  const params = useParams<{ id?: string }>();
  const store = useStore();
  const self = useSelf(store);
  const players = usePlayers(store);
  void store.getGames();

  return <Suspense fallback={<Loading />}>{wrapRender(params.id, self(), players())}</Suspense>;
};

const wrapRender = (param?: string, self?: number, players?: PlayerType[]) => {
  if (self === undefined || players === undefined) {
    return <></>;
  }
  const games = useGames();

  const navigate = useNavigate();
  const id = getId(navigate, self, param);

  if (isNaN(id)) {
    navigate('/player', { replace: true });
  }

  const playerPosition = players.findIndex(p => p.id === id);

  return (
    <Show when={playerPosition >= 0} fallback={<NotFound />}>
      <div class='router-player'>
        <PlayerHeader
          self={id === self}
          player={players[playerPosition]}
          position={playerPosition + 1}
          players={players.length}
        />
        <PlayerHeader
          self={id === self}
          player={players[playerPosition]}
          position={0}
          players={players.length}
        />
        <PlayerStats player={players[playerPosition]} />
        <Suspense>{wrapGames(id, players, games())}</Suspense>
      </div>
    </Show>
  );
};

const PlayerHeader = (props: {
  self: boolean;
  player: PlayerType;
  position: number;
  players: number;
}) => {
  const [name, setName] = createSignal(props.player.name);
  const [editigName, setEditingName] = createSignal(false);

  return (
    <div class='router-player-header'>
      <Switch>
        <Match when={props.position === 1}>
          <span class='router-player-header-badge first'>
            <icon.Crown />
          </span>
        </Match>
        <Match when={props.position === 2}>
          <span class='router-player-header-badge second'>
            <icon.Medal />
          </span>
        </Match>
        <Match when={props.position === 3}>
          <span class='router-player-header-badge third'>
            <icon.Certificate />
          </span>
        </Match>
        <Match when={props.position === props.players - 3}>
          <span class='router-player-header-badge'>
            <icon.Mosquito />
          </span>
        </Match>
        <Match when={props.position === props.players - 2}>
          <span class='router-player-header-badge'>
            <icon.Poop />
          </span>
        </Match>
        <Match when={props.position === props.players - 1}>
          <span class='router-player-header-badge'>
            <icon.Worm />
          </span>
        </Match>
        <Match when={props.position === props.players - 0}>
          <span class='router-player-header-badge'>
            <icon.Skull />
          </span>
        </Match>
      </Switch>
      <span class='router-player-header-name'>{props.player.name}</span>
      <span class='router-player-header-score'># {props.position}</span>
    </div>
  );
};

const PlayerStats = (props: { player: PlayerType }) => (
  <div class='router-player-stats'>
    <b>Wins</b>
    {props.player.wins}
    <b>Losses</b>
    {props.player.losses}
    <b>Points won</b>
    {props.player.pointsWon}
    <b>Points lost</b>
    {props.player.pointsLost}
    <b>Joined</b>
    {props.player.createdMs}
  </div>
);

const NotFound = () => {
  return (
    <div class='router-player-not-found'>
      <div class='router-player-not-found-icon'>
        <icon.Magnifier />
      </div>
      <h1>Player not found</h1>
    </div>
  );
};

const getId = (navigate: Navigator, self: number, param?: string) => {
  if (param === undefined) {
    return self;
  }

  const id = Number(param);
  if (isNaN(id)) {
    navigate('/player', { replace: true });
  }

  return id;
};

const wrapGames = (id: number, players: PlayerType[], games?: Game[]) => {
  console.debug('id:', id);
  console.debug('players:', players);
  console.debug('games:', games);

  console.debug('memo games:', games);

  return (
    <Show when={games !== undefined}>
      <Games
        players={players}
        games={games?.filter(g => g.playerOne === id || g.playerTwo === id)}
      />
    </Show>
  );
};
