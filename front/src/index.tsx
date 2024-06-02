/* @refresh reload */
import { render } from 'solid-js/web'
import { ParentProps } from 'solid-js'

import { Side } from './components';
import { Router } from './router';
import { Store, WithStore } from './store';

import './index.css'

const store = new Store();
const root = document.getElementById('root')

const App = (props: ParentProps) =>
  <>
    <Side />
    <div>{props.children}</div>
  </>;

render(() =>
  <WithStore store={store}>
    <Router root={App} />
  </WithStore>,
  root!
);
