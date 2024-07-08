import { createMemo, createSignal, For } from 'solid-js';

import { Prompt } from './prompt';
import { Store } from '../../store';
import { type Player } from '../../types';
import { type Getter } from '../../util';

export const Game = (props: {
  store: Store;
  self: Getter<Player>;
  opponents: Getter<Opponent[]>;
  hide: () => void;
}) => {
  const [score, setScore] = createSignal(11);
  const [opponent, setOpponent] = createSignal(0);
  const [opponentScore, setOpponentScore] = createSignal(0);
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

        props.store.registerGame(opponentId, score(), opponentScore()).then(() => {
          props.hide();
        });
      }}
      cancel={() => {
        console.debug('Cancel');
        props.hide();
      }}
      disabled={invalidScores}
    >
      <>{console.debug(props.opponents())}</>
      <p>
        <b>{props.self()?.name}</b>
        <select onInput={e => setScore(e.target.selectedIndex)} value={score()}>
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
      </p>
      <p>
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
        <select onInput={e => setOpponentScore(e.target.selectedIndex)} value={opponentScore()}>
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
      </p>
    </Prompt>
  );
};

type Opponent = Pick<Player, 'id' | 'name'>;
