/* @refresh reload */
import { render } from 'solid-js/web'
import { Show, DEV, ParentProps } from 'solid-js'

import { Ribbon, Nav } from './components';
import { Router } from './router';
import { Store, WithStore } from './store';

import './index.css'

const store = new Store();
const root = document.getElementById('root')

const App = (props: ParentProps) =>
  <>
    <Show when={DEV}>
      <Ribbon text='Development' />
    </Show>
    <Nav />
    {props.children}
  </>;

render(() =>
  <WithStore store={store}>
    <Router root={App} />
  </WithStore>,
  root!
);
