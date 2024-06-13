import { Match, Show, Suspense, Switch, createMemo, createSignal } from 'solid-js';
import { Navigator, useNavigate, useParams } from '@solidjs/router';

import { usePlayers, useSelf, useStore } from '../store';
import { type Player as PlayerType } from '../types';
import { icon, Loading } from '../components';

import './player.css';

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

const PlayerView = (props: {
  self: boolean;
  player: PlayerType;
  position: number;
  players: number;
}) => {
  return (
    <>
      <h1>{props.self && 'Me'}</h1>
      <h1>
        <Switch>
          <Match when={props.position === 1}>
            <span class='router-player-badge first'>
              <icon.Crown />
            </span>
          </Match>
          <Match when={props.position === 2}>
            <span class='router-player-badge second'>
              <icon.Medal />
            </span>
          </Match>
          <Match when={props.position === 3}>
            <span class='router-player-badge third'>
              <icon.Certificate />
            </span>
          </Match>
          <Match when={props.position === props.players - 3}>
            <span class='router-player-badge'>
              <icon.Mosquito />
            </span>
          </Match>
          <Match when={props.position === props.players - 2}>
            <span class='router-player-badge'>
              <icon.Poop />
            </span>
          </Match>
          <Match when={props.position === props.players - 1}>
            <span class='router-player-badge'>
              <icon.Worm />
            </span>
          </Match>
          <Match when={props.position === props.players - 0}>
            <span class='router-player-badge'>
              <icon.Skull />
            </span>
          </Match>
        </Switch>
        {props.position} {props.player.name}
      </h1>
      <p>{props.player.email}</p>
    </>
  );
};

const wrapRender = (param?: string, self?: number, players?: PlayerType[]) => {
  if (self === undefined || players === undefined) {
    return <></>;
  }

  const navigate = useNavigate();
  const id = getId(navigate, self, param);

  if (isNaN(id)) {
    navigate('/player', { replace: true });
  }

  const playerPosition = players.findIndex(p => p.id === id);

  return (
    <Show when={playerPosition >= 0} fallback={<NotFound />}>
      <PlayerView
        self={id === self}
        player={players[playerPosition]}
        position={playerPosition + 1}
        players={players.length}
      />
    </Show>
  );
};

export const Player = () => {
  const params = useParams<{ id?: string }>();
  const store = useStore();
  const self = useSelf(store);
  const players = usePlayers(store);
  // TODO: preload matches
  // void store.getMatches();

  return <Suspense fallback={<Loading />}>{wrapRender(params.id, self(), players())}</Suspense>;
};
