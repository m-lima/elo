import { createSignal, createEffect, onCleanup, Suspense } from 'solid-js';

import { icon } from '../components';
import { useSelf, useStore } from '../store';

import './home.css';

export const Home = () => {
  const [count, setCount] = createSignal(0);
  const [prev, setPrev] = createSignal(0);
  const [bla, setBla] = useBla();
  bloink();

  const store = useStore();
  const self = useSelf();
  console.log('building home');

  return (
    <div class='router-home'>
      <h1>Vite + Solid</h1>
      <icon.User /> Player
      <div>
        <button
          onClick={() => {
            setPrev(count());
            setCount(count => count + 1);
          }}
        >
          count is {count()}
        </button>
        <button
          onClick={() => {
            setBla(bla() + 1);
            store.increment();
          }}
        >
          bla is {bla()}
        </button>
        <p>
          <icon.Spinner />
          loading
        </p>
        <p>But was {prev()}</p>
        <p>And bla {bla()}</p>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <a href='https://www.google.com'>Skull</a>
      <a href='https://unviseted.place.pongelo'>Skull</a>
      <h1>
        <icon.Trophy /> Bla
        <br />
        <icon.Spinner /> Loading
      </h1>
      <ul style={{ 'font-size': '3em' }}>
        <li>
          <span style={{ color: 'gold' }}>
            <icon.Crown />
          </span>{' '}
          Crown
        </li>
        <li>
          <span style={{ color: 'white' }}>
            <icon.Medal />
          </span>{' '}
          Medal
        </li>
        <li>
          <span style={{ color: 'burlywood' }}>
            <icon.Certificate />
          </span>{' '}
          Certificate
        </li>
        <li>
          <icon.Mosquito /> Mosquito
        </li>
        <li>
          <icon.Poop /> Poop
        </li>
        <li>
          <icon.Worm /> Worm
        </li>
        <li>
          <icon.Skull /> Skull
        </li>
      </ul>
      <div>
        <Suspense>
          <h1>Player</h1>
          <h3>Id</h3>
          {self()?.id}
          <h3>Name</h3>
          {self()?.name}
          <h3>Email</h3>
          {self()?.email}
        </Suspense>
      </div>
    </div>
  );
};

const useBla = () => {
  const signal = createSignal(3);

  createEffect(() => {
    console.log('Register', signal[0]());
    onCleanup(() => {
      console.log('Clean');
    });
  });

  return signal;
};

const bloink = () => {
  const data = {
    id: 27,
    name: 'namer',
    value: 3,
  };

  const { id, ...rest } = data;

  console.log(id, rest);
};
