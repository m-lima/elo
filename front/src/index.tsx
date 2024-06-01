/* @refresh reload */
import { render } from 'solid-js/web'
import { Show, DEV, ParentProps } from 'solid-js'

import { Ribbon, Nav } from './components';
import { Router } from './router';

import './index.css'

const root = document.getElementById('root')

const App = (props: ParentProps) =>
  <>
    <Show when={DEV}>
      <Ribbon text='Development' />
    </Show>
    <Nav />
    {props.children}
  </>;

render(() => <Router root={App} />, root!)
