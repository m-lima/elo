import { Accessor, createMemo, createSignal, For, Setter } from 'solid-js';

import { Store } from '../../store';
import { type Game, type Getter, type Player } from '../../types';

import { Prompt, type Props } from './prompt';

import './edit.css';

export const Edit = (
  props: Props & {
    store: Store;
    players: Getter<Player[]>;
    game: Game;
  },
) => {
  const [busy, setBusy] = createSignal<boolean | undefined>();

  const [player, setPlayer] = createSignal(props.game.playerOne);
  const [opponent, setOpponent] = createSignal(props.game.playerTwo);

  const [score, setScore] = createSignal(props.game.scoreOne);
  const [opponentScore, setOpponentScore] = createSignal(props.game.scoreTwo);
  const [challenge, setChallenge] = createSignal(props.game.challenge);

  const players = createMemo(() =>
    props
      .players()
      ?.map(p => {
        return { id: p.id, name: p.name };
      })
      .sort((a, b) => a.name.localeCompare(b.name)),
  );

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
      title='New game'
      visible={props.visible}
      ok={() => {
        setTimeout(() => setBusy(busy => busy ?? true), 200);
        props.store
          .editGame({
            ...props.game,
            playerOne: player(),
            playerTwo: opponent(),
            scoreOne: score(),
            scoreTwo: opponentScore(),
            challenge: challenge(),
          })
          .then(r => {
            if (r) {
              props.hide();
            }
          })
          .finally(() => {
            setBusy(false);
            setTimeout(setBusy, 500);
          });
      }}
      cancel={() => {
        props.hide();
      }}
      disabled={() => invalidPlayers() || invalidScores()}
      busy={busy}
    >
      <div class='components-prompt-edit'>
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
  set: Setter<number>;
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
