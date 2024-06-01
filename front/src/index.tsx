/* @refresh reload */
import { render } from 'solid-js/web'
import { ParentProps } from 'solid-js'

import { Nav } from './components';
import { Router } from './router';
import { Store, WithStore } from './store';

import './index.scss'

const store = new Store();
const root = document.getElementById('root')

const App = (props: ParentProps) =>
  <>
    <Nav />
    {props.children}
  </>;

render(() =>
  <WithStore store={store}>
    <Router root={App} />
  </WithStore>,
  root!
);
