import { createMemo, createSignal, For } from 'solid-js';

import { Store } from '../../store';
import { type Player, type Game as GameType } from '../../types';
import { type Getter } from '../../util';

import { Prompt, type Props } from './prompt';

// TODO: Reorganize this prompt
export const Game = (
  props: Props & {
    store: Store;
    self: Getter<Player>;
    players: Getter<Player[]>;
    games: Getter<GameType[]>;
  },
) => {
  const [score, setScore] = createSignal(11);
  const [opponent, setOpponent] = createSignal(0);
  const [opponentScore, setOpponentScore] = createSignal(0);
  const [challenge, setChallenge] = createSignal(false);

  const shown = createMemo(prev => {
    if (prev === true) {
      return true;
    } else {
      return props.visible();
    }
  });

  const opponents = createMemo(() => {
    if (!shown()) {
      return [];
    }

    const innerSelf = props.self()?.id;
    const innerGames = props.games();
    const innerPlayers = props.players();

    if (innerSelf === undefined || innerGames === undefined || innerPlayers === undefined) {
      return [];
    }

    return buildOpponentList(innerGames, innerPlayers, innerSelf);
  });

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
        const opponentId = opponents()[opponent()]?.id;
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
          onInput={e => setOpponent(e.target.selectedIndex)}
          value={opponents()[opponent()]?.id}
        >
          <For each={opponents()}>{o => <option value={o.id}>{o.name}</option>}</For>
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

const buildOpponentList = (games: GameType[], players: Player[], self: number) => {
  return Array.from(
    games
      .map(g => {
        if (g.playerOne === self) {
          return g.playerTwo;
        } else if (g.playerTwo === self) {
          return g.playerOne;
        } else {
          return;
        }
      })
      .reduce(
        (acc, curr) => {
          if (curr !== undefined) {
            const entry = acc.get(curr);
            if (entry !== undefined) {
              acc.set(curr, { name: entry.name, count: entry.count + 1 });
            }
          }
          return acc;
        },
        new Map(players.filter(p => p.id !== self).map(p => [p.id, { name: p.name, count: 0 }])),
      )
      .entries(),
  )
    .sort((a, b) => b[1].count - a[1].count)
    .map(o => {
      return { id: o[0], name: o[1].name };
    });
};
