import { Accessor, createMemo, createSignal, For, Setter } from 'solid-js';

import { Store } from '../../store';
import { type Getter, type Player, type User } from '../../types';

import { Prompt, type Props } from './prompt';

import './game.css';

export const Game = (
  props: Props & {
    store: Store;
    players: Getter<Player[]>;
    self: Getter<User>;
    opponent?: Getter<User>;
  },
) => {
  const [maybePlayer, setPlayer] = createSignal(props.self()?.id);
  const [maybeOpponent, setOpponent] = createSignal(props.opponent?.()?.id);

  const [score, setScore] = createSignal(11);
  const [opponentScore, setOpponentScore] = createSignal(0);
  const [challenge, setChallenge] = createSignal(false);

  const players = createMemo(() =>
    props
      .players()
      ?.map(p => {
        return { id: p.id, name: p.name };
      })
      .sort((a, b) => a.name.localeCompare(b.name)),
  );

  const player = createMemo(() => maybePlayer() ?? props.self()?.id);
  const opponent = createMemo(() => maybeOpponent() ?? props.opponent?.()?.id);

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
        const playerInner = player();
        if (playerInner === undefined) {
          return;
        }

        const opponentInner = opponent();
        if (opponentInner === undefined) {
          return;
        }

        props.store
          .registerGame(playerInner, opponentInner, score(), opponentScore(), challenge())
          .then(r => {
            if (r) {
              props.hide();
            }
          });
      }}
      cancel={() => {
        props.hide();
      }}
      disabled={() =>
        invalidPlayers() || invalidScores() || player() === undefined || opponent() === undefined
      }
    >
      <div class='components-prompt-game'>
        <PlayerList get={player} set={setPlayer} players={players} invalid={invalidPlayers} />
        <Score get={score} set={setScore} invalid={invalidScores} />
        <PlayerList get={opponent} set={setOpponent} players={players} invalid={invalidPlayers} />
        <Score get={opponentScore} set={setOpponentScore} invalid={invalidScores} />
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
  get: Getter<number>;
  set: Setter<number | undefined>;
  players: Getter<SimplePlayer[]>;
  invalid: Accessor<boolean>;
}) => (
  <select
    class={props.invalid() ? 'invalid' : undefined}
    value={props.get()}
    onInput={e => props.set(Number(e.currentTarget.value))}
  >
    <For each={props.players()}>{o => <option value={o.id}>{o.name}</option>}</For>
  </select>
);

const Score = (props: {
  get: Accessor<number>;
  set: Setter<number>;
  invalid: Accessor<boolean>;
}) => (
  <select
    class={props.invalid() ? 'invalid' : undefined}
    value={props.get()}
    onInput={e => props.set(e.target.selectedIndex)}
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
