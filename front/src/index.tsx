/* @refresh reload */
import { render } from 'solid-js/web'
import { Show, DEV } from 'solid-js'

import { Ribbon } from './components/mod';
import { Root } from './route/mod';

import './index.css'

const root = document.getElementById('root')

const App = () =>
  <>
    <Show when={DEV}>
      <Ribbon text='Development' />
    </Show>
    <Root />
  </>;

render(() => <App />, root!)
