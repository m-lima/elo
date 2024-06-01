import { createSignal, createEffect, onCleanup, Suspense } from 'solid-js'

import solidLogo from '../assets/solid.svg'
import viteLogo from '/vite.svg'

import './home.css'
import { useSelf, useStore } from '../store';

export const Home = () => {
  const [count, setCount] = createSignal(0);
  const [prev, setPrev] = createSignal(0);
  const [bla, setBla] = useBla();
  bloink();

  const store = useStore();
  const self = useSelf(store);

  return (
    <div class='router home container'>
      <div class='router home inner'>
        <div>
          <a href='https://vitejs.dev' target='_blank'>
            <img src={viteLogo} class='router home logo' alt='Vite logo' />
          </a>
          <a href='https://solidjs.com' target='_blank'>
            <img src={solidLogo} class='router home logo solid' alt='Solid logo' />
          </a>
        </div>
        <h1>Vite + Solid</h1>
        <div class='router home card'>
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
        <p class='router home read-the-docs'>
          Click on the Vite and Solid logos to learn more
        </p>
        <Suspense fallback={<h1>Loading</h1>} >
          <div>
            <h1>User</h1>
            <h3>Id</h3>
            {self()?.id}
            <h3>Name</h3>
            {self()?.name}
            <h3>Email</h3>
            {self()?.email}
          </div>
        </Suspense>
      </div>
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
