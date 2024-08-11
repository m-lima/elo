import { Accessor, createEffect, createSignal, For, Setter } from 'solid-js';

import { icon } from '.';
import { date } from '../util';

import './datepicker.css';

export const DatePicker = (props: { getter: Accessor<Date>; setter: Setter<Date> }) => {
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
    <div class='components-datepicker'>
      <div class='header'>
        <span class='clickable' onClick={() => setYear(y => y - 1)}>
          <icon.DoubleLeft />
        </span>
        <span class='clickable' onClick={() => setMonth(m => m - 1)}>
          <icon.Left />
        </span>
        <b>{date.monthToString(month())}</b>
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
                  newDate.setSeconds(0);
                  newDate.setMilliseconds(0);
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
                  newDate.setSeconds(0);
                  newDate.setMilliseconds(0);
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
