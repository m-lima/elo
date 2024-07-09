import { createMemo, createSignal, Match, Show, Suspense, Switch } from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';

import { error, Loading, Main } from '../pages';
import { action, icon, prompt, Games } from '../components';
import { type Player as PlayerType } from '../types';
import { Store, useStore } from '../store';
import { type EnrichedPlayer, type Getter, compareLists, enrichPlayers } from '../util';

import './player.css';

enum Prompt {
  Invite,
  Rename,
  Game,
}

export const Player = () => {
  const params = useParams<{ id?: string }>();
  const navigate = useNavigate();
  const store = useStore();
  const games = store.getGames();
  const players = store.getPlayers();
  const invites = store.getInvites();
  const self = store.getSelf();

  const [visiblePrompt, setVisiblePrompt] = createSignal<Prompt | undefined>();

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

  const player = createMemo(() => {
    const enrichedPlayers = enrichPlayers(players(), games());
    const inviteCount =
      (players()?.filter(p => p.inviter !== undefined && p.inviter === id()).length ?? 0) +
      (invites()?.filter(p => p.inviter === id()).length ?? 0);

    const position = enrichedPlayers.findIndex(p => p.id === id());
    const player = { invites: inviteCount, ...enrichedPlayers[position] };

    return { position: position + 1, player };
  });

  return (
    <Suspense fallback=<Loading />>
      <Show when={player().position > 0} fallback=<error.NotFound />>
        <prompt.Invite
          visible={() => visiblePrompt() === Prompt.Invite}
          hide={setVisiblePrompt}
          store={store}
          players={players}
          invites={invites}
        />
        <prompt.Rename
          visible={() => visiblePrompt() === Prompt.Rename}
          hide={setVisiblePrompt}
          store={store}
          name={player().player.name}
          players={players}
          invites={invites}
        />
        <prompt.Game
          visible={() => visiblePrompt() === Prompt.Game}
          hide={setVisiblePrompt}
          store={store}
          self={() => player().player}
          players={players}
          games={games}
        />
        <action.Actions>
          <Switch>
            <Match when={id() === self()?.id}>
              <action.Rename action={() => setVisiblePrompt(Prompt.Rename)} />
            </Match>
            <Match when={id() !== self()?.id}>
              <action.Game action={() => setVisiblePrompt(Prompt.Game)} />
            </Match>
          </Switch>
          <action.Invite action={() => setVisiblePrompt(Prompt.Invite)} />
        </action.Actions>
        <Main>
          <div class='routes-player' id='main'>
            <PlayerHeader
              self={id() === self()?.id}
              player={player().player}
              position={player().position}
              players={players.length}
            />
            <PlayerStats player={player().player} />
            <Suspense
              fallback=<div>
                <icon.Spinner /> Loading games
              </div>
            >
              <GameList store={store} players={players} id={id} />
            </Suspense>
          </div>
        </Main>
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

const PlayerStats = (props: { player: EnrichedPlayer & { invites: number } }) => (
  <div class='routes-player-stats'>
    <b>Games</b>
    {props.player.games}
    <b>Wins</b>
    {props.player.wins}
    <b>Losses</b>
    {props.player.losses}
    <b>Challenges won</b>
    {props.player.challengesWon}
    <b>Challenges lost</b>
    {props.player.challengesLost}
    <b>Points won</b>
    {props.player.pointsWon}
    <b>Points lost</b>
    {props.player.pointsLost}
    <b>Joined</b>
    {props.player.createdMs}
    <b>Invites</b>
    {props.player.invites}
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
