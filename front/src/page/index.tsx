import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { Loading } from '../components';
import { Unauthorized } from './error';
import { Welcome } from './welcome';
import { useSelf } from '../store';

export * as error from './error';

export const Page = (props: ParentProps<{ state: state.State }>) => {
  const self = useSelf();

  return (
    <ErrorBoundary
      fallback={error => {
        console.log('INNER CAUGHT', error);
        return <h1>{JSON.stringify(error)}</h1>;
      }}
    >
      <Suspense fallback={<Loading />}>
        <Show when={self()?.pending !== true} fallback={<Welcome />}>
          <Switch fallback={<div>{props.children}</div>}>
            <Match when={props.state === state.Disconnected.Connecting}>
              <Loading />
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
