import { Accessor, createMemo, createSignal, For, Setter, Show } from 'solid-js';

import { Store } from '../../store';
import { type Getter, type Player } from '../../types';
import { date } from '../../util';
import { DatePicker } from '..';

import { Prompt, type Props } from './prompt';

import './register.css';

export const Register = (
  props: Props & {
    store: Store;
    players: Getter<Player[]>;
    self: Getter<number>;
    opponent?: Getter<number>;
  },
) => {
  const [busy, setBusy] = createSignal<boolean | undefined>();

  const [maybePlayer, setPlayer] = createSignal(props.self());
  const [maybeOpponent, setOpponent] = createSignal(props.opponent?.());

  const [score, setScore] = createSignal(11);
  const [opponentScore, setOpponentScore] = createSignal(0);
  const [challenge, setChallenge] = createSignal(false);
  const [millis, setMillis] = createSignal<Date | undefined>();

  const [datepickerVisible, setDatepickerVisible] = createSignal(false);

  const players = createMemo(() =>
    props
      .players()
      ?.map(p => {
        return { id: p.id, name: p.name };
      })
      .sort((a, b) => a.name.localeCompare(b.name)),
  );

  const player = createMemo(() => maybePlayer() ?? props.self());
  const opponent = createMemo(() => maybeOpponent() ?? props.opponent?.());

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

        setTimeout(() => setBusy(busy => busy ?? true), 200);
        props.store
          .registerGame(
            playerInner,
            opponentInner,
            score(),
            opponentScore(),
            challenge(),
            millis() ?? new Date(),
          )
          .then(r => {
            if (r) {
              setDatepickerVisible(false);
              setMillis();
              props.hide();
            }
          })
          .finally(() => {
            setBusy(false);
            setTimeout(setBusy, 500);
          });
      }}
      cancel={() => {
        setDatepickerVisible(false);
        setMillis();
        props.hide();
      }}
      disabled={() =>
        invalidPlayers() || invalidScores() || player() === undefined || opponent() === undefined
      }
      busy={busy}
    >
      <div class='components-prompt-register'>
        <PlayerList get={player} set={setPlayer} players={players} invalid={invalidPlayers} />
        <Score get={score} set={setScore} invalid={invalidScores} />
        <PlayerList get={opponent} set={setOpponent} players={players} invalid={invalidPlayers} />
        <Score get={opponentScore} set={setOpponentScore} invalid={invalidScores} />
        <span
          classList={{ datepicker: true, active: datepickerVisible() }}
          onClick={() => setDatepickerVisible(v => !v)}
        >
          {date.toLongString(millis() ?? new Date())}
        </span>
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
      <Show when={datepickerVisible()}>
        <DatePicker
          getter={() => millis() ?? new Date()}
          setter={setMillis}
          hide={() => setDatepickerVisible(false)}
        />
      </Show>
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
