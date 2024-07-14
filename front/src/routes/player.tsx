import {
  createMemo,
  createSignal,
  onCleanup,
  onMount,
  type Accessor,
  Match,
  Show,
  Suspense,
  Switch,
} from 'solid-js';
import { useNavigate, useParams } from '@solidjs/router';
import { Line } from 'solid-chartjs';
import { Chart, Filler, ScriptableLineSegmentContext, Title, Tooltip } from 'chart.js';

import { error, Loading, Main } from '../pages';
import { action, Games, icon, More, prompt } from '../components';
import { type Getter, type EnrichedPlayer, type EnrichedGame } from '../types';
import { useStore } from '../store';
import { monthToString } from '../util';
import * as consts from '../consts';

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
  const players = store.useEnrichedPlayers();
  const games = store.useEnrichedGames();
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

  const playerGames = createMemo(() =>
    games()
      .filter(g => g.playerOne === id() || g.playerTwo === id())
      .map(g => {
        if (g.playerTwo === id()) {
          return {
            ...g,
            playerOne: g.playerTwo,
            playerTwo: g.playerOne,
            balanceOne: g.balanceTwo,
            balanceTwo: g.balanceOne,
            scoreOne: g.scoreTwo,
            scoreTwo: g.scoreOne,
            ratingOne: g.ratingTwo,
            ratingTwo: g.ratingOne,
            ratingDelta: g.ratingDelta !== undefined ? -g.ratingDelta : undefined,
            playerOneName: g.playerTwoName,
            playerTwoName: g.playerOneName,
          };
        } else {
          return g;
        }
      }),
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
            <PlayerHeader player={player} playerCount={players().length ?? 0} />
            <PlayerStats player={player} />
            <Show when={playerGames().length > 0} fallback=<error.NotGames inline />>
              <Charts games={playerGames} />
              <Games games={playerGames} />
            </Show>
          </div>
        </Main>
      </Show>
    </Suspense>
  );
};

const PlayerHeader = (props: { player: Getter<EnrichedPlayer>; playerCount: number }) => (
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
    <span class='routes-player-header-name-score'>
      <span class='routes-player-header-name'>{props.player()?.name}</span>
      <span class='routes-player-header-score'># {props.player()?.position}</span>
    </span>
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

const Charts = (props: { games: Accessor<EnrichedGame[]> }) => {
  const [responsive, setResponsive] = createSignal(false);
  // TODO: Use this limit
  const [limit, setLimit] = createSignal(100);

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

  const games = createMemo(
    () => {
      return Array.from(props.games().filter((_, i) => i < limit()))
        .reverse()
        .map(g => {
          return {
            balance: g.balanceOne,
            rating: g.ratingOne,
            pointsWon: g.scoreOne,
            pointsLost: g.scoreTwo,
            challenge: g.challenge,
            createdMs: g.createdMs,
          };
        });
    },
    [],
    { equals: false },
  );

  return (
    <>
      <div id='routes-player-chart-rating'>
        <Line
          height={300}
          data={{
            labels: games().map(g => dateToString(new Date(g.createdMs))),
            datasets: [
              {
                label: 'Rating',
                data: games().map(g => g.rating),
                cubicInterpolationMode: 'monotone',
                backgroundColor: consts.colors.accentSemiTransparent,
                borderColor: consts.colors.accent,
                pointBackgroundColor: games().map(g =>
                  g.challenge ? 'white' : consts.colors.accentSemiTransparent,
                ),
                pointBorderColor: games().map(g => (g.challenge ? 'white' : consts.colors.accent)),
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
            labels: games().map(g => dateToString(new Date(g.createdMs))),
            datasets: [
              {
                label: 'Balance',
                data: games().map(g => g.balance),
                cubicInterpolationMode: 'monotone',
                backgroundColor: '#80808080',
                borderColor: '#808080',
                pointStyle: false,
                segment: {
                  borderColor: (ctx: ScriptableLineSegmentContext) =>
                    ctx.p0.parsed.y > ctx.p1.parsed.y ? '#a03030' : '#30a030',
                },
              },
              {
                label: 'Points won',
                data: games().map(g => g.pointsWon),
                cubicInterpolationMode: 'monotone',
                backgroundColor: '#30a03080',
                showLine: false,
                pointStyle: false,
                fill: 'origin',
                yAxisID: 'balance',
              },
              {
                label: 'Points lost',
                data: games().map(g => -g.pointsLost),
                cubicInterpolationMode: 'monotone',
                backgroundColor: '#a0303080',
                showLine: false,
                pointStyle: false,
                fill: 'origin',
                yAxisID: 'balance',
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
                  text: 'Score',
                },
              },
              balance: {
                display: false,
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
