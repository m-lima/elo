import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { Unauthorized } from './error';
import { Welcome } from './welcome';
import { Loading } from './loading';
import { useSelf } from '../store';

export const Wrapper = (props: ParentProps<{ state: state.State }>) => {
  const self = useSelf();

  return (
    <Switch fallback={<div>{props.children}</div>}>
      <Match when={props.state === state.Disconnected.Connecting}>
        <Loading />
      </Match>
      <Match when={props.state === state.Disconnected.Unauthorized}>
        <Unauthorized />
      </Match>
      <Match when={state.isDisconnected(props.state)}>
        <div>Disconnected</div>
      </Match>
      <Match when={true}>
        <ErrorBoundary
          fallback={error => {
            console.debug('INNER CAUGHT', error);
            if ('millis' in error) {
              return <div>Timed out {error.millis / 1000}s</div>;
            }

            return <div>Something went wrong</div>;
          }}
        >
          <Suspense fallback={<Loading />}>
            <Show when={self()?.pending !== true} fallback={<Welcome />}>
              <div>{props.children}</div>
            </Show>
          </Suspense>
        </ErrorBoundary>
      </Match>
    </Switch>
  );
};
