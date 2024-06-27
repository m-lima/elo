import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { Unauthorized } from './error';
import { Welcome } from './welcome';
import { Loading } from './loading';
import { useSelf } from '../store';

export const Wrapper = (props: ParentProps<{ state: state.State }>) => {
  const self = useSelf();

  return (
    <ErrorBoundary
      fallback={error => {
        console.log('INNER CAUGHT', error);
        return <h1>{JSON.stringify(error)}</h1>;
      }}
    >
      <Suspense
        fallback={
          <div>
            Suspense
            <Loading />
          </div>
        }
      >
        <Show when={self()?.pending !== true} fallback={<Welcome />}>
          <Switch fallback={<div>{props.children}</div>}>
            <Match when={props.state === state.Disconnected.Connecting}>
              <div>
                Match top
                <Loading />
              </div>
            </Match>
            <Match when={props.state === state.Disconnected.Unauthorized}>
              <Unauthorized />
            </Match>
            <Match when={state.isDisconnected(props.state)}>
              <div></div>
            </Match>
          </Switch>
        </Show>
      </Suspense>
    </ErrorBoundary>
  );
};
