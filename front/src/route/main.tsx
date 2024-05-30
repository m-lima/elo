import { createSignal, createEffect, onCleanup } from 'solid-js'
import type { ParentProps } from 'solid-js';

import solidLogo from '../assets/solid.svg'
import viteLogo from '/vite.svg'

import './main.css'

export const Main = (props: ParentProps) => {
  const [count, setCount] = createSignal(0);
  const [prev, setPrev] = createSignal(0);
  const [bla, setBla] = useBla();
  bloink();

  return (
    <div class='main'>
      <div>
        <a href='https://vitejs.dev' target='_blank'>
          <img src={viteLogo} class='logo' alt='Vite logo' />
        </a>
        <a href='https://solidjs.com' target='_blank'>
          <img src={solidLogo} class='logo solid' alt='Solid logo' />
        </a>
      </div>
      <h1>Vite + Solid</h1>
      <div class='card'>
        <button onClick={() => { setPrev(count()); setCount((count) => count + 1) }}>
          count is {count()}
        </button>
        <button onClick={() => { setBla(bla() + 1) }}>
          bla is {bla()}
        </button>
        <p>
          But was {prev()}
        </p>
        <p>
          And bla {bla()}
        </p>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p class='read-the-docs'>
        Click on the Vite and Solid logos to learn more
      </p>
      {props.children}
    </div>
  )
};

const useBla = () => {
  const signal = createSignal(3);

  createEffect(() => {
    console.log('Register', signal[0]())
    onCleanup(() => console.log('Clean'));
  });

  return signal;
};

const bloink = () => {
  let data = {
    id: 27,
    name: 'namer',
    value: 3,
  };

  let { id, ...rest } = data;

  console.log(id, rest)
};
