/* @refresh reload */
import { render } from 'solid-js/web';
import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch, createSignal } from 'solid-js';

import { Router } from './router';
import { status, error, Side, Loading, Invite } from './components';
import { Store, WithStore, useSelf } from './store';
import { state } from './socket';

import './index.css';

const root = document.getElementById('root');

const socket = Store.makeSocket('ws://localhost:3333/ws/binary', 'http://localhost:3333/check');
const [socketState, setSocketState] = createSignal(socket.getState());
socket.registerStateListener(state => setSocketState(state));

const store = new Store(socket);

const InviteWrnapper = (props: ParentProps) => {
  const self = useSelf(store);
  console.debug('Current self:', self());

  return (
    <Suspense fallback={<Loading />}>
      <Show when={self()?.pending === false} fallback={<Invite />}>
        <div>{props.children}</div>
      </Show>
    </Suspense>
  );
};

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
        <Switch fallback={<InviteWrnapper>{props.children}</InviteWrnapper>}>
          <Match when={socketState() === state.Disconnected.Connecting}>
            <Loading />
          </Match>
          <Match when={socketState() === state.Disconnected.Unauthorized}>
            <error.Unauthorized />
          </Match>
          <Match when={state.isDisconnected(socketState())}>
            <div></div>
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
