import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { TimeOut, GenericError, Unauthorized } from './error';
import { Welcome } from './welcome';
import { Loading } from './loading';
import { useStore } from '../store';

export const Wrapper = (props: ParentProps<{ state: state.State }>) => {
  const self = useStore().getSelf();

  return (
    <Switch>
      <Match when={props.state === state.Disconnected.Connecting}>
        <Loading />
      </Match>
      <Match when={props.state === state.Disconnected.Unauthorized}>
        <Unauthorized />
      </Match>
      <Match when={state.isDisconnected(props.state)}>
        <GenericError />
      </Match>
      <Match when={true}>
        <ErrorBoundary fallback={error => ('millis' in error ? <TimeOut /> : <GenericError />)}>
          <Suspense fallback=<Loading />>
            <Show when={self()?.pending !== true} fallback=<Welcome />>
              {props.children}
            </Show>
          </Suspense>
        </ErrorBoundary>
      </Match>
    </Switch>
  );
};
