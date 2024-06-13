/* @refresh reload */
import { render } from 'solid-js/web';
import { ErrorBoundary, Match, ParentProps, Switch, createSignal } from 'solid-js';

import { Router } from './router';
import { status, error, Side, Loading } from './components';
import { Store, WithStore } from './store';
import { state } from './socket';

import './index.css';

const socket = Store.makeSocket('ws://localhost:3333/ws/binary', 'http://localhost:3333/check');
const [socketState, setSocketState] = createSignal(socket.getState());
socket.registerStateListener(state => setSocketState(state));

const store = new Store(socket);
const root = document.getElementById('root');

const App = (props: ParentProps) => {
  console.debug('Current state:', state.toString(socketState()));
  return (
    <>
      <Side />
      <ErrorBoundary
        fallback={error => {
          console.log('INNER CAUGHT', error);
          return <h1>{JSON.stringify(error)}</h1>;
        }}
      >
        <Switch>
          <Match when={state.isPending(socketState())}>
            <Loading />
          </Match>
          <Match when={state.isConnected(socketState())}>
            <div>{props.children}</div>
          </Match>
          <Match when={socketState() === state.Disconnected.Unauthorized}>
            <error.Unauthorized />
          </Match>
          <Match when={socketState() === state.Disconnected.Error}>
            <error.Unauthorized />
          </Match>
          <Match when={socketState() === state.Disconnected.Closed}>
            <error.Unauthorized />
          </Match>
        </Switch>
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
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  root!,
);
