import { createMemo, createSignal, For } from 'solid-js';

import { Store } from '../../store';
import { type Player } from '../../types';
import { type Getter } from '../../util';

import { Prompt, type Props } from './prompt';

// TODO: Reorganize this prompt
export const Game = (
  props: Props & {
    store: Store;
    self: Getter<Player>;
    opponents: Getter<Opponent[]>;
  },
) => {
  const [score, setScore] = createSignal(11);
  const [opponent, setOpponent] = createSignal(0);
  const [opponentScore, setOpponentScore] = createSignal(0);
  const [challenge, setChallenge] = createSignal(false);

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
      ok={() => {
        const opponentId = props.opponents()?.[opponent()]?.id;
        if (opponentId === undefined) {
          return;
        }

        props.store.registerGame(opponentId, score(), opponentScore(), challenge()).then(r => {
          if (r) {
            props.hide();
          }
        });
      }}
      cancel={() => {
        console.debug('Cancel');
        props.hide();
      }}
      disabled={invalidScores}
    >
      <div>
        <b>{props.self()?.name}</b>
        <select
          class={invalidScores() ? 'invalid' : undefined}
          onInput={e => setScore(e.target.selectedIndex)}
          value={score()}
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
      </div>
      <div>
        <b>Opponent</b>
        <select
          onInput={e => {
            console.debug(props.opponents()?.[e.target.selectedIndex]);
            setOpponent(e.target.selectedIndex);
          }}
          value={props.opponents()?.[opponent()]?.id}
        >
          <For each={props.opponents()}>{o => <option value={o.id}>{o.name}</option>}</For>
        </select>
        <select
          class={invalidScores() ? 'invalid' : undefined}
          onInput={e => setOpponentScore(e.target.selectedIndex)}
          value={opponentScore()}
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
      </div>
      <input
        type='checkbox'
        checked={challenge()}
        onChange={e => setChallenge(e.currentTarget.checked)}
      />
    </Prompt>
  );
};

type Opponent = Pick<Player, 'id' | 'name'>;
