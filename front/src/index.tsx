/* @refresh reload */
import { render } from 'solid-js/web';
import { ErrorBoundary, Match, ParentProps, Show, Switch, createSignal } from 'solid-js';

import { Router } from './router';
import { status, Side, Loading } from './components';
import { Message, Request, Store, WithStore } from './store';
import { Socket, state } from './socket';

import './index.css';

const socket = new Socket<Request, Message>(
  'ws://localhost:3333/ws/binary',
  'http://localhost:3333/check',
);
const [socketState, setSocketState] = createSignal(socket.getState());
socket.registerStateListener(state => setSocketState(state));

const store = new Store(socket);
const root = document.getElementById('root');

const App = (props: ParentProps) => {
  return (
    <>
      <Side />
      <ErrorBoundary
        fallback={error => {
          console.log('INNER CAUGHT', error);
          return <h1>{JSON.stringify(error)}</h1>;
        }}
      >
        <Show when={state.isConnected(socketState())} fallback={<Loading />}>
          <div>{props.children}</div>
        </Show>
      </ErrorBoundary>
    </>
  );
};

// TODO: Better global fallback
render(
  () => {
    return (
      <WithStore store={store}>
        <Switch>
          <Match when={socketState() === state.Disconnected.Connecting}>
            <status.Connecting />
          </Match>
          <Match when={socketState() === state.Connected.Fetching}>
            <status.Loading />
          </Match>
        </Switch>
        <Router root={App} />
      </WithStore>
    );
  },
  // Allowed because this is normal solid construct
  /* eslint-disable-next-line
@typescript-eslint/no-non-null-assertion
*/
  root!,
);
