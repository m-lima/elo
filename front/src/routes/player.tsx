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
import { type Getter, type EnrichedPlayer } from '../types';
import { useStore } from '../store';
import { monthToString } from '../util';
import * as consts from '../consts';

import './player.css';

enum Prompt {
  Invite,
  Rename,
  Game,
}

// TODO: The graph is unhappy with a reload of this page
export const Player = () => {
  const params = useParams<{ id?: string }>();
  const navigate = useNavigate();
  const store = useStore();
  const games = store.useGames();
  const players = store.useEnrichedPlayers();
  const invites = store.useInvites();
  const self = store.useSelf();

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
    const enrichedPlayers = players();

    const player = enrichedPlayers.find(p => p.id === id());
    if (player !== undefined) {
      const inviteCount =
        (players().filter(p => p.inviter !== undefined && p.inviter === id()).length ?? 0) +
        (invites()?.filter(p => p.inviter === id()).length ?? 0);

      return { invites: inviteCount, ...player };
    }
  });

  const playerGames = createMemo(
    () => {
      const innerGames = games();
      const innerId = id();
      if (innerGames === undefined || innerId === undefined) {
        return [];
      }

      let balance = 0;
      return innerGames
        .filter(g => g.playerOne === innerId || g.playerTwo === innerId)
        .map(g => {
          if (g.playerOne === innerId) {
            return {
              rating: g.ratingOne,
              pointsWon: g.scoreOne,
              pointsLost: g.scoreTwo,
              challenge: g.challenge,
              createdMs: g.createdMs,
            };
          } else {
            return {
              rating: g.ratingTwo,
              pointsWon: g.scoreTwo,
              pointsLost: g.scoreOne,
              challenge: g.challenge,
              createdMs: g.createdMs,
            };
          }
        })
        .reverse()
        .map(g => {
          balance += g.pointsWon - g.pointsLost;
          return {
            balance,
            ...g,
          };
        });
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
              playerCount={players().length ?? 0}
            />
            <PlayerStats player={player} />
            <Show when={playerGames().length > 0}>
              <Charts games={playerGames} player={player} />
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

const Charts = (props: { games: Accessor<PlayerGame[]>; player: Getter<EnrichedPlayer> }) => {
  const [responsive, setResponsive] = createSignal(false);

  onMount(() => {
    Chart.register(Title, Tooltip, Filler);

    // ChartJS was trying to get the size of the parent component, but SolidJS
    // only constructs the elements and not necessarily add it to the DOM.
    //
    // Since there is no lifecycle hook for being added to the DOM, this
    // timeout hack does the trick
    const schedule = () => {
      setTimeout(() => {
        if (
          !!document.getElementById('routes-player-chart-rating') &&
          !!document.getElementById('routes-player-chart-score')
        ) {
          setResponsive(true);
        } else {
          schedule();
        }
      }, 0);
    };
    schedule();
  });

  onCleanup(() => {
    Chart.unregister(Title, Tooltip, Filler);
  });

  return (
    <>
      <div id='routes-player-chart-rating'>
        <Line
          height={300}
          data={{
            labels: props.games().map(g => dateToString(new Date(g.createdMs))),
            datasets: [
              {
                label: 'Rating',
                data: props.games().map(g => g.rating),
                cubicInterpolationMode: 'monotone',
                backgroundColor: consts.colors.accentSemiTransparent,
                borderColor: consts.colors.accent,
                pointBackgroundColor: props
                  .games()
                  .map(g => (g.challenge ? 'white' : consts.colors.accentSemiTransparent)),
                pointBorderColor: props
                  .games()
                  .map(g => (g.challenge ? 'white' : consts.colors.accent)),
              },
            ],
          }}
          options={{
            responsive: responsive(),
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
      <div id='routes-player-chart-score'>
        <Line
          height={300}
          data={{
            labels: props.games().map(g => dateToString(new Date(g.createdMs))),
            datasets: [
              {
                label: 'Balance',
                data: props.games().map(g => g.balance),
                cubicInterpolationMode: 'monotone',
                backgroundColor: '#80808080',
                borderColor: '#808080',
                pointStyle: false,
                segment: {
                  borderColor: (ctx: ScriptableLineSegmentContext) =>
                    ctx.p0.parsed.y > ctx.p1.parsed.y ? '#a03030' : '#30a030',
                },
                yAxisID: 'balance',
              },
              {
                label: 'Points won',
                data: props.games().map(g => g.pointsWon),
                cubicInterpolationMode: 'monotone',
                backgroundColor: '#30a03080',
                showLine: false,
                pointStyle: false,
                fill: 'origin',
              },
              {
                label: 'Points lost',
                data: props.games().map(g => -g.pointsLost),
                cubicInterpolationMode: 'monotone',
                backgroundColor: '#a0303080',
                showLine: false,
                pointStyle: false,
                fill: 'origin',
              },
            ],
          }}
          options={{
            responsive: responsive(),
            maintainAspectRatio: false,
            interaction: {
              mode: 'index',
              intersect: false,
            },
            scales: {
              y: {
                title: {
                  display: true,
                  text: 'Points',
                },
              },
              balance: {
                title: {
                  display: true,
                  text: 'Point balance',
                },
                position: 'right',
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

type PlayerGame = {
  readonly rating: number;
  readonly balance: number;
  readonly pointsWon: number;
  readonly pointsLost: number;
  readonly challenge: boolean;
  readonly createdMs: number;
};
