import { Accessor, createMemo, createSignal, For, Setter } from 'solid-js';

import { Store } from '../../store';
import { type Getter, type Player, type Game as GameType } from '../../types';

import { Prompt, type Props } from './prompt';

import './game.css';

export const Game = (
  props: Props & {
    store: Store;
    self: Getter<Player>;
    players: Getter<Player[]>;
    games: Getter<GameType[]>;
  },
) => {
  const [maybePlayer, setPlayer] = createSignal<number | undefined>();
  const [maybeOpponent, setOpponent] = createSignal<number | undefined>();

  const [score, setScore] = createSignal(11);
  const [opponentScore, setOpponentScore] = createSignal(0);
  const [challenge, setChallenge] = createSignal(false);

  const players = createMemo(() => {
    const playersInner = props.players();
    if (playersInner === undefined) {
      return [];
    }
    return playersInner
      .map(p => {
        return { id: p.id, name: p.name };
      })
      .sort((a, b) => a.name.localeCompare(b.name));
  });

  const player = createMemo(() => maybePlayer() ?? props.self()?.id);

  const opponents = createMemo(
    () => {
      const innerPlayer = player();
      if (innerPlayer === undefined) {
        return players();
      }

      return players().filter(p => p.id !== innerPlayer);
    },
    players(),
    { equals: false },
  );

  const opponent = createMemo(() => {
    const innerPlayer = player();
    const opponentInner = maybeOpponent();
    if (opponentInner !== undefined && opponentInner !== innerPlayer) {
      return opponentInner;
    }

    const opponentsInner = opponents();
    return opponentsInner.length === 0 ? undefined : opponentsInner[0].id;
  });

  const invalidPlayers = createMemo(() => player() === opponent());

  const invalidScores = createMemo(() => {
    if (score() === opponentScore()) {
      return true;
    }

    if (score() === 11) {
      return opponentScore() >= 11;
    }

    if (score() === 12) {
      return opponentScore() !== 10;
    }

    if (opponentScore() === 11) {
      return score() >= 11;
    }

    if (opponentScore() === 12) {
      return score() !== 10;
    }

    return true;
  });

  return (
    <Prompt
      visible={props.visible}
      ok={() => {
        const opponentInner = opponent();
        if (opponentInner === undefined) {
          return;
        }

        props.store.registerGame(opponentInner, score(), opponentScore(), challenge()).then(r => {
          if (r) {
            props.hide();
          }
        });
      }}
      cancel={() => {
        props.hide();
      }}
      disabled={() => invalidPlayers() || invalidScores()}
    >
      <div class='components-prompt-game'>
        <PlayerList
          getter={player}
          setter={setPlayer}
          players={props.players}
          invalid={invalidPlayers}
        />
        <Score getter={score} setter={setScore} invalid={invalidScores} />
        <PlayerList
          getter={opponent}
          setter={setOpponent}
          players={opponents}
          invalid={invalidPlayers}
        />
        <Score getter={opponentScore} setter={setOpponentScore} invalid={invalidScores} />
        <label for='challenge' class='checkbox-label' onClick={() => setChallenge(c => !c)}>
          Challenge
        </label>
        <input
          type='checkbox'
          checked={challenge()}
          onChange={e => setChallenge(e.currentTarget.checked)}
          name='challenge'
        />
      </div>
    </Prompt>
  );
};

const PlayerList = (props: {
  getter: Getter<number>;
  setter: Setter<number | undefined>;
  players: Getter<SimplePlayer[]>;
  invalid: Accessor<boolean>;
}) => (
  <select
    class={props.invalid() ? 'invalid' : undefined}
    value={props.getter()}
    onInput={e => props.setter(props.setter(Number(e.currentTarget.value)))}
  >
    <For each={props.players()}>{o => <option value={o.id}>{o.name}</option>}</For>
  </select>
);

const Score = (props: {
  getter: Accessor<number>;
  setter: Setter<number>;
  invalid: Accessor<boolean>;
}) => (
  <select
    class={props.invalid() ? 'invalid' : undefined}
    value={props.getter()}
    onInput={e => props.setter(e.target.selectedIndex)}
  >
    <option value={0}>0</option>
    <option value={1}>1</option>
    <option value={2}>2</option>
    <option value={3}>3</option>
    <option value={4}>4</option>
    <option value={5}>5</option>
    <option value={6}>6</option>
    <option value={7}>7</option>
    <option value={8}>8</option>
    <option value={9}>9</option>
    <option value={10}>10</option>
    <option value={11}>11</option>
    <option value={12}>12</option>
  </select>
);

type SimplePlayer = {
  readonly id: number;
  readonly name: string;
};
