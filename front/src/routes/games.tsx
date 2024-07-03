import { For, Show, Suspense, createMemo, createSignal } from 'solid-js';

import { Store, useStore } from '../store';
import { icon, Action, Games as GameTable, Actions } from '../components';
import { Loading, Main, Prompt } from '../pages';
import { type Getter } from '../util';
import { type Player } from '../types';

export const Games = () => {
  const store = useStore();
  const self = store.getSelf();
  const games = store.getGames();
  const players = store.getPlayers();
  const [prompt, setPrompt] = createSignal(false);
  const shown = createMemo(prev => {
    if (prev === true) {
      return true;
    } else {
      return prompt();
    }
  });
  const opponents = createMemo(() => {
    if (!shown()) {
      return [];
    }

    const innerSelf = self();
    const innerGames = games();
    const innerPlayers = players();

    if (innerSelf === undefined || innerGames === undefined || innerPlayers === undefined) {
      return [];
    }

    return Array.from(
      innerGames
        .map(g => {
          if (g.playerOne === innerSelf.id) {
            return g.playerTwo;
          } else if (g.playerTwo === innerSelf.id) {
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
          new Map(
            innerPlayers
              .filter(p => p.id !== innerSelf.id)
              .map(p => [p.id, { name: p.name, count: 0 }]),
          ),
        )
        .entries(),
    )
      .sort((a, b) => b[1].count - a[1].count)
      .map(o => {
        return { id: o[0], name: o[1].name };
      });
  });

  return (
    <Suspense fallback=<Loading />>
      <Show when={prompt()}>
        <GamePrompt
          store={store}
          self={() => players()?.find(p => p.id === self()?.id)}
          opponents={opponents}
          hide={() => setPrompt(false)}
        />
      </Show>
      <Actions>
        <Action
          icon=<icon.Add />
          text='New game'
          action={() => {
            setPrompt(true);
            // void store.registerGame(7, 17, 21);
          }}
        />
      </Actions>
      <Main>
        <GameTable players={players} games={games} />
      </Main>
    </Suspense>
  );
};

const GamePrompt = (props: {
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
