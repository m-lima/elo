import { Accessor, Match, Switch } from 'solid-js';

import { icon } from '.';
import { state } from '../socket';

import './status.css';

export const Status = (props: { state: Accessor<state.State> }) => (
  <Switch>
    <Match when={props.state() === state.Disconnected.Connecting}>
      <Connecting />
    </Match>
    <Match when={props.state() === state.Connected.Fetching}>
      <Loading />
    </Match>
  </Switch>
);

const Connecting = () => {
  return (
    <div class='components-status'>
      <icon.Spinner /> Connecting
    </div>
  );
};

const Loading = () => {
  return (
    <div class='components-status'>
      <icon.Spinner /> Loading
    </div>
  );
};
