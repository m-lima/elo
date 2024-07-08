import { createMemo, Match, Show, Suspense, Switch } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';

import { error, Loading, Main } from '../pages';
import { icon, Games, Action, Actions } from '../components';
import { type Player as PlayerType } from '../types';
import { Store, useStore } from '../store';
import { compareLists, type Getter } from '../util';

import './player.css';

export const Player = () => {
  const params = useParams<{ id?: string }>();
  const navigate = useNavigate();
  const store = useStore();
  const rawPlayers = store.getPlayers();
  const self = store.getSelf();

  // Preload games
  void store.getGames();

  const id = createMemo(() => {
    if (params.id === undefined) {
      return self()?.id;
    }

    if (params.id.trim() !== '') {
      const parsed = Number(params.id);
      if (!Number.isNaN(parsed)) {
        return parsed;
      }
    }

    navigate('/player', { replace: true });
    return;
  });

  const players = createMemo(() => {
    const players = rawPlayers();
    return players === undefined ? [] : players;
  });

  const playerPosition = createMemo(() => {
    return players().findIndex(p => p.id === id());
  });

  return (
    <Suspense fallback=<Loading />>
      <Show when={playerPosition() >= 0} fallback=<error.NotFound />>
        <>
          <Actions>
            <Action
              icon=<icon.Add />
              text='Invite'
              action={() => {
                store.invitePlayer('new player', 'player@email.com');
                console.debug('Clicked');
              }}
            />
            <Switch>
              <Match when={id() === self()?.id}>
                <Action
                  icon=<icon.Edit />
                  text='Name'
                  action={() => {
                    console.debug('Clicked');
                  }}
                />
              </Match>
              <Match when={id() !== self()?.id}>
                <Action
                  icon=<icon.Add />
                  text='Game'
                  action={() => {
                    console.debug('Clicked');
                  }}
                />
              </Match>
            </Switch>
          </Actions>
          <Main>
            <div class='routes-player' id='main'>
              <PlayerHeader
                self={id() === self()?.id}
                player={players()[playerPosition()]}
                position={playerPosition() + 1}
                players={players.length}
              />
              <PlayerStats player={players()[playerPosition()]} />
              <Suspense
                fallback=<div>
                  <icon.Spinner /> Loading games
                </div>
              >
                <GameList store={store} players={players} id={id} />
              </Suspense>
            </div>
          </Main>
        </>
      </Show>
    </Suspense>
  );
};

const PlayerHeader = (props: {
  self: boolean;
  player: PlayerType;
  position: number;
  players: number;
}) => (
  <div class='routes-player-header'>
    <Switch>
      <Match when={props.position === 1}>
        <span class='routes-player-header-badge first'>
          <icon.Crown />
        </span>
      </Match>
      <Match when={props.position === 2}>
        <span class='routes-player-header-badge second'>
          <icon.Medal />
        </span>
      </Match>
      <Match when={props.position === 3}>
        <span class='routes-player-header-badge third'>
          <icon.Certificate />
        </span>
      </Match>
      <Match when={props.position === props.players - 3}>
        <span class='routes-player-header-badge'>
          <icon.Mosquito />
        </span>
      </Match>
      <Match when={props.position === props.players - 2}>
        <span class='routes-player-header-badge'>
          <icon.Poop />
        </span>
      </Match>
      <Match when={props.position === props.players - 1}>
        <span class='routes-player-header-badge'>
          <icon.Worm />
        </span>
      </Match>
      <Match when={props.position === props.players - 0}>
        <span class='routes-player-header-badge'>
          <icon.Skull />
        </span>
      </Match>
    </Switch>
    <span class='routes-player-header-name'>{props.player.name}</span>
    <span class='routes-player-header-score'># {props.position}</span>
  </div>
);

const PlayerStats = (props: { player: PlayerType }) => (
  <div class='routes-player-stats'>
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

const GameList = (props: { store: Store; players: Getter<PlayerType[]>; id: Getter<number> }) => {
  const rawGames = props.store.getGames();
  const games = createMemo(
    () => {
      const games = rawGames();
      const ided = props.id();
      return games === undefined || ided === undefined
        ? []
        : games.filter(g => g.playerOne === ided || g.playerTwo === ided);
    },
    [],
    { equals: compareLists },
  );

  return <Games players={props.players} games={games} />;
};
