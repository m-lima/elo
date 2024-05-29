/* @refresh reload */
import { render } from 'solid-js/web'
import { Show, DEV } from 'solid-js'
import { Ribbon } from './components/mod';
import { Router } from './route/mod';

import './index.css'

const root = document.getElementById('root')

const App = () =>
  <>
    <Show when={DEV}>
      <Ribbon text='Development' />
    </Show>
    <Router />
  </>;

render(() => <App />, root!)
