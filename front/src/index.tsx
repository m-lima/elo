/* @refresh reload */
import { render } from 'solid-js/web'
import { ParentProps } from 'solid-js'

import { Nav, Side } from './components';
import { Router } from './router';
import { Store, WithStore } from './store';

import './index.css'

const store = new Store();
const root = document.getElementById('root')

const App = (props: ParentProps) =>
  <div class='app holder nav'>
    <Nav />
    <div class='app holder side'>
      <Side />
      {props.children}
    </div>
  </div>;

render(() =>
  <WithStore store={store}>
    <Router root={App} />
  </WithStore>,
  root!
);
