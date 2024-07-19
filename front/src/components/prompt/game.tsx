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
    opponent?: Getter<Player>;
  },
) => {
  const [score, setScore] = createSignal(11);
  const [maybeOpponent, setOpponent] = createSignal<number | undefined>();
  const [opponentScore, setOpponentScore] = createSignal(0);
  const [challenge, setChallenge] = createSignal(false);

  const selfName = createMemo(() => props.players()?.find(p => p.id === props.self()?.id)?.name);

  const opponents = createMemo(() => {
    const playersInner = props.players();
    if (playersInner === undefined) {
      return [];
    }
    return playersInner
      .map(p => {
        return { id: p.id, name: p.name };
      })
      .filter(p => p.id !== props.self()?.id)
      .sort((a, b) => a.name.localeCompare(b.name));
  });

  const invalidScore = createMemo(() => {
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

  const opponent = createMemo(() => maybeOpponent() ?? props.opponent?.()?.id);

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
      disabled={() => invalidScore() || opponent() === undefined}
    >
      <div class='components-prompt-game'>
        <div class='components-prompt-game-self'>{selfName()}</div>
        <Score getter={score} setter={setScore} invalid={invalidScore} />
        <select value={opponent()} onInput={e => setOpponent(Number(e.currentTarget.value))}>
          <For each={opponents()}>{o => <option value={o.id}>{o.name}</option>}</For>
        </select>
        <Score getter={opponentScore} setter={setOpponentScore} invalid={invalidScore} />
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
