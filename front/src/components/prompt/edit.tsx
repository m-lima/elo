import { Accessor, createEffect, createMemo, createSignal, For, Setter } from 'solid-js';

import { Store } from '../../store';
import { type Game, type Getter, type Player } from '../../types';
import { icon } from '..';
import * as util from '../../util';

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
  const [deleted, setDeleted] = createSignal(props.game.deleted);
  const [millis, setMillis] = createSignal(new Date());

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
      visible={props.visible}
      ok={() => {
        setTimeout(() => setBusy(busy => busy ?? true), 200);
        const change = deleted()
          ? {
              ...props.game,
              deleted: true,
            }
          : {
              ...props.game,
              playerOne: player(),
              playerTwo: opponent(),
              scoreOne: score(),
              scoreTwo: opponentScore(),
              challenge: challenge(),
              deleted: false,
            };

        props.store
          .editGame(change)
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
      disabled={() => !deleted() && (invalidPlayers() || invalidScores())}
      busy={busy}
    >
      <div class='components-prompt-edit'>
        <PlayerList
          get={player}
          set={setPlayer}
          players={players}
          invalid={invalidPlayers}
          deleted={deleted}
        />
        <Score get={score} set={setScore} invalid={invalidScores} deleted={deleted} />
        <PlayerList
          get={opponent}
          set={setOpponent}
          players={players}
          invalid={invalidPlayers}
          deleted={deleted}
        />
        <Score
          get={opponentScore}
          set={setOpponentScore}
          invalid={invalidScores}
          deleted={deleted}
        />
        <label
          for='challenge'
          classList={{
            'checkbox-label': true,
            'disabled': deleted(),
          }}
          onClick={() => {
            if (!deleted()) {
              setChallenge(c => !c);
            }
          }}
        >
          Challenge
        </label>
        <input
          type='checkbox'
          checked={challenge()}
          onChange={e => setChallenge(e.currentTarget.checked)}
          name='challenge'
          disabled={deleted()}
        />
        <button
          classList={{
            delete: true,
            toggle: true,
            active: deleted(),
          }}
          onClick={() => setDeleted(d => !d)}
        >
          <icon.Trash />
          <span> Delete</span>
        </button>
      </div>
      <DatePicker getter={millis} setter={setMillis} />
    </Prompt>
  );
};

const DatePicker = (props: { getter: Accessor<Date>; setter: Setter<Date> }) => {
  const now = new Date();

  const [month, setMonth] = createSignal(props.getter().getMonth());
  const [year, setYear] = createSignal(props.getter().getFullYear());

  const [hourRef, setHourRef] = createSignal<Element | undefined>();
  const [minuteRef, setMinuteRef] = createSignal<Element | undefined>();

  createEffect(() => {
    hourRef()?.scrollIntoView({ behavior: 'instant', block: 'center' });
    minuteRef()?.scrollIntoView({ behavior: 'instant', block: 'center' });
  });

  return (
    <div class='datepicker'>
      <div class='header'>
        <span class='clickable' onClick={() => setYear(y => y - 1)}>
          <icon.DoubleLeft />
        </span>
        <span class='clickable' onClick={() => setMonth(m => m - 1)}>
          <icon.Left />
        </span>
        <b>{util.date.monthToString(month())}</b>
        <b>{year()}</b>
        <span class='clickable' onClick={() => setMonth(m => m + 1)}>
          <icon.Right />
        </span>
        <span class='clickable' onClick={() => setYear(y => y + 1)}>
          <icon.DoubleRight />
        </span>
        <span
          class='clickable reset'
          onClick={() => {
            setMonth(now.getMonth());
            setYear(now.getFullYear());
            hourRef()?.scrollIntoView({ behavior: 'instant', block: 'center' });
            minuteRef()?.scrollIntoView({ behavior: 'instant', block: 'center' });
            props.setter(now);
          }}
        >
          <icon.Now />
        </span>
      </div>
      <div class='date pickable'>
        <span class='weekday'>Sun</span>
        <span class='weekday'>Mon</span>
        <span class='weekday'>Tue</span>
        <span class='weekday'>Wed</span>
        <span class='weekday'>Thu</span>
        <span class='weekday'>Fri</span>
        <span class='weekday'>Sat</span>
        <For each={getDaysOfMonth(year(), month())}>
          {d => (
            <span
              classList={{
                item: true,
                clickable: true,
                now: sameDay(d, now),
                selected: sameDay(d, props.getter()),
                disabled: d.getMonth() !== month(),
              }}
              onClick={() =>
                props.setter(old => {
                  const newDate = new Date(d);
                  newDate.setHours(old.getHours());
                  newDate.setMinutes(old.getMinutes());
                  return newDate;
                })
              }
            >
              {d.getDate()}
            </span>
          )}
        </For>
      </div>
      <div class='hours pickable'>
        <For each={Array.from(Array(24).keys())}>
          {h => (
            <span
              classList={{
                item: true,
                clickable: true,
                now: now.getHours() === h,
                selected: props.getter().getHours() === h,
              }}
              onClick={() =>
                props.setter(old => {
                  const newDate = new Date(old);
                  newDate.setHours(h);
                  return newDate;
                })
              }
              ref={props.getter().getHours() === h ? setHourRef : undefined}
            >
              {String(h).padStart(2, '0')}
            </span>
          )}
        </For>
      </div>
      <div class='minutes pickable'>
        <For each={Array.from(Array(60).keys())}>
          {m => (
            <span
              classList={{
                item: true,
                clickable: true,
                now: now.getMinutes() === m,
                selected: props.getter().getMinutes() === m,
              }}
              onClick={() =>
                props.setter(old => {
                  const newDate = new Date(old);
                  newDate.setMinutes(m);
                  return newDate;
                })
              }
              ref={props.getter().getMinutes() === m ? setMinuteRef : undefined}
            >
              {String(m).padStart(2, '0')}
            </span>
          )}
        </For>
      </div>
    </div>
  );
};

const getDaysOfMonth = (year: number, month: number) => {
  const date = new Date(year, month, 1);
  const days = [];
  let day = 1 - date.getDay();
  let current = new Date(year, month, day);

  while (day < 42 && !(current.getMonth() > month && current.getDay() === 0)) {
    days.push(current);
    day++;
    current = new Date(year, month, day);
  }

  return days;
};

const sameDay = (date: Date, other: Date) =>
  date.getFullYear() === other.getFullYear() &&
  date.getMonth() === other.getMonth() &&
  date.getDate() === other.getDate();

const PlayerList = (props: {
  get: Getter<number>;
  set: Setter<number>;
  players: Getter<SimplePlayer[]>;
  invalid: Accessor<boolean>;
  deleted: Accessor<boolean>;
}) => (
  <select
    class={props.invalid() ? 'invalid' : undefined}
    value={props.get()}
    onInput={e => props.set(Number(e.currentTarget.value))}
    disabled={props.deleted()}
  >
    <For each={props.players()}>{o => <option value={o.id}>{o.name}</option>}</For>
  </select>
);

const Score = (props: {
  get: Accessor<number>;
  set: Setter<number>;
  invalid: Accessor<boolean>;
  deleted: Accessor<boolean>;
}) => (
  <select
    class={props.invalid() ? 'invalid' : undefined}
    value={props.get()}
    onInput={e => props.set(e.target.selectedIndex)}
    disabled={props.deleted()}
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
