/* @refresh reload */
import { render } from 'solid-js/web';
import { ParentProps, createSignal } from 'solid-js';

import { Router } from './router';
import { Status, Side } from './components';
import { Store, WithStore } from './store';

import './index.css';
import { Wrapper } from './page';

const root = document.getElementById('root');

const socket = Store.makeSocket('ws://localhost:3333/ws/binary', 'http://localhost:3333/check');
const [socketState, setSocketState] = createSignal(socket.getState());
socket.registerStateListener(state => setSocketState(state));

const store = new Store(socket);

const App = (props: ParentProps) => (
  <>
    <Side />
    <Wrapper state={socketState()}>
      <div>{props.children}</div>
    </Wrapper>
  </>
);

render(
  () => {
    return (
      <WithStore store={store}>
        <Status state={socketState()} />
        <Router root={App} />
      </WithStore>
    );
  },
  // Allowed because this is normal solid construct
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  root!,
);
