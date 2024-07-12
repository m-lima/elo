/* @refresh reload */
import { render } from 'solid-js/web';
import { ParentProps, createSignal } from 'solid-js';

import { Routes } from './routes';
import { Notifications, Status, Side } from './components';
import { Store, WithStore } from './store';
import { Wrapper } from './pages';

import './index.css';

const root = document.getElementById('root');

const socket = Store.makeSocket(
  `ws://${location.hostname}:3333/ws/binary`,
  `http://${location.hostname}:3333/check`,
);
const [socketState, setSocketState] = createSignal(socket.getState(), { equals: false });
socket.registerStateListener(setSocketState);

const store = new Store(socket);

const App = (props: ParentProps) => (
  <>
    <Side />
    <Wrapper state={socketState()}>{props.children}</Wrapper>
  </>
);

render(
  () => {
    return (
      <WithStore store={store}>
        <Status state={socketState} />
        <Notifications />
        <Routes root={App} />
      </WithStore>
    );
  },
  // Allowed because this is normal solid construct
  // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
  root!,
);
