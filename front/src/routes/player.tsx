import {
  createMemo,
  createSignal,
  Match,
  type Accessor,
  onMount,
  Show,
  Suspense,
  Switch,
  onCleanup,
} from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { Line } from 'solid-chartjs';
import { Chart, Filler, ScriptableLineSegmentContext, Title, Tooltip } from 'chart.js';

import { error, Loading, Main } from '../pages';
import { action, icon, prompt, Games } from '../components';
import { type Game } from '../types';
import { useStore } from '../store';
import { type EnrichedPlayer, type Getter, enrichPlayers, monthToString, colors } from '../util';

import './player.css';

enum Prompt {
  Invite,
  Rename,
  Game,
}

// TODO: Loading keeps showing for this page
// TODO: The graph is unhappy with a reload of this page
// TODO: Show points gained/lost in the graph
// TODO: Make the chart shorter
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

    const player = enrichedPlayers.find(p => p.id === id());
    if (player !== undefined) {
      const inviteCount =
        (players()?.filter(p => p.inviter !== undefined && p.inviter === id()).length ?? 0) +
        (invites()?.filter(p => p.inviter === id()).length ?? 0);

      return { invites: inviteCount, ...player };
    }
  });

  const filteredGames = createMemo(
    () => {
      const innerGames = games();
      const innerId = id();
      return innerGames === undefined || innerId === undefined
        ? []
        : innerGames.filter(g => g.playerOne === innerId || g.playerTwo === innerId);
    },
    [],
    { equals: false },
  );

  return (
    <Suspense fallback=<Loading />>
      <Show when={player() !== undefined} fallback=<error.NotFound />>
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
          name={player()?.name ?? ''}
          players={players}
          invites={invites}
        />
        <prompt.Game
          visible={() => visiblePrompt() === Prompt.Game}
          hide={setVisiblePrompt}
          store={store}
          self={player}
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
              player={player}
              playerCount={players()?.length ?? 0}
            />
            <PlayerStats player={player} />
            <Show when={filteredGames().length > 0}>
              <Charts id={id} games={filteredGames} player={player} />
            </Show>
            <Games players={players} games={games} player={id} />
          </div>
        </Main>
      </Show>
    </Suspense>
  );
};

const PlayerHeader = (props: {
  self: boolean;
  player: Getter<EnrichedPlayer>;
  playerCount: number;
}) => (
  <div class='routes-player-header'>
    <Switch>
      <Match when={props.player()?.position === 1}>
        <span class='routes-player-header-badge first'>
          <icon.Crown />
        </span>
      </Match>
      <Match when={props.player()?.position === 2}>
        <span class='routes-player-header-badge second'>
          <icon.Medal />
        </span>
      </Match>
      <Match when={props.player()?.position === 3}>
        <span class='routes-player-header-badge third'>
          <icon.Certificate />
        </span>
      </Match>
      <Match when={props.player()?.position === props.playerCount - 3}>
        <span class='routes-player-header-badge'>
          <icon.Mosquito />
        </span>
      </Match>
      <Match when={props.player()?.position === props.playerCount - 2}>
        <span class='routes-player-header-badge'>
          <icon.Poop />
        </span>
      </Match>
      <Match when={props.player()?.position === props.playerCount - 1}>
        <span class='routes-player-header-badge'>
          <icon.Worm />
        </span>
      </Match>
      <Match when={props.player()?.position === props.playerCount - 0}>
        <span class='routes-player-header-badge'>
          <icon.Skull />
        </span>
      </Match>
    </Switch>
    <span class='routes-player-header-name'>{props.player()?.name}</span>
    <span class='routes-player-header-score'># {props.player()?.position}</span>
  </div>
);

const PlayerStats = (props: { player: Getter<EnrichedPlayer & { invites: number }> }) => (
  <div class='routes-player-stats'>
    <b>Games</b>
    {props.player()?.games}
    <b>Rating</b>
    {props.player()?.rating.toFixed(2)}
    <b>Wins</b>
    {props.player()?.wins}
    <b>Losses</b>
    {props.player()?.losses}
    <b>Challenges won</b>
    {props.player()?.challengesWon}
    <b>Challenges lost</b>
    {props.player()?.challengesLost}
    <b>Points won</b>
    {props.player()?.pointsWon}
    <b>Points lost</b>
    {props.player()?.pointsLost}
  </div>
);

const Charts = (props: {
  id: Getter<number>;
  games: Accessor<Game[]>;
  player: Getter<EnrichedPlayer>;
}) => {
  onMount(() => {
    Chart.register(Title, Tooltip, Filler);
  });

  onCleanup(() => {
    Chart.unregister(Title, Tooltip, Filler);
  });

  const games = createMemo(
    () => {
      const id = props.id();
      if (id === undefined) {
        return [];
      }

      const innerGames = props.games();
      const games = Array<{ rating: number; balance: number; createdMs: number }>(
        innerGames.length,
      );

      let balance = 0;

      for (let i = 0; i < innerGames.length - 1; i++) {
        const game = innerGames[i];
        const next = innerGames[i + 1];
        const rating = next.playerOne === id ? next.ratingOne : next.ratingTwo;
        if (game.playerOne === id) {
          balance += game.scoreOne - game.scoreTwo;
          games[i] = {
            rating,
            balance,
            createdMs: game.createdMs,
          };
        } else {
          balance += game.scoreTwo - game.scoreOne;
          games[i] = {
            rating,
            balance,
            createdMs: game.createdMs,
          };
        }
      }

      const last = innerGames.pop();
      if (last !== undefined) {
        if (last.playerOne === id) {
          games[innerGames.length] = {
            rating: props.player()?.rating ?? 0,
            balance: balance + last.scoreOne - last.scoreTwo,
            createdMs: last.createdMs,
          };
        } else {
          games[innerGames.length] = {
            rating: props.player()?.rating ?? 0,
            balance: balance + last.scoreTwo - last.scoreOne,
            createdMs: last.createdMs,
          };
        }
      }

      return games;
    },
    [],
    { equals: false },
  );

  return (
    <>
      <div>
        <Line
          data={{
            labels: games().map(g => dateToString(new Date(g.createdMs))),
            datasets: [
              {
                label: 'Rating',
                data: games().map(g => g.rating),
                cubicInterpolationMode: 'monotone',
                backgroundColor: colors.accent + '80',
                borderColor: colors.accent,
              },
            ],
          }}
          options={{
            responsive: true,
            maintainAspectRatio: false,
            interaction: {
              mode: 'index',
              intersect: false,
            },
            scales: {
              y: {
                title: {
                  display: true,
                  text: 'Rating',
                },
              },
            },
          }}
        />
      </div>
      <div>
        <Line
          data={{
            labels: games().map(g => dateToString(new Date(g.createdMs))),
            datasets: [
              {
                label: 'Balance',
                data: games().map(g => g.balance),
                cubicInterpolationMode: 'monotone',
                backgroundColor: colors.accent + '80',
                borderColor: colors.accent,
                pointStyle: false,
                segment: {
                  borderColor: (ctx: ScriptableLineSegmentContext) =>
                    ctx.p0.parsed.y > ctx.p1.parsed.y ? '#a03030' : '#30a030',
                },
                // fill: {
                //   above: '#30a03080',
                //   below: '#a0303080',
                //   target: 'origin',
                // },
              },
            ],
          }}
          options={{
            responsive: true,
            maintainAspectRatio: false,
            interaction: {
              mode: 'index',
              intersect: false,
            },
            scales: {
              y: {
                title: {
                  display: true,
                  text: 'Point balance',
                },
              },
            },
          }}
        />
      </div>
    </>
  );
};

const dateToString = (date: Date) =>
  `${String(date.getDate()).padStart(2, '0')}/${monthToString(date.getMonth())}/${String(date.getFullYear() % 1000).padStart(2, '0')} `;
