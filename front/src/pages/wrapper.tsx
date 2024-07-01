import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { TimeOut, GenericError, Unauthorized } from './error';
import { Welcome } from './welcome';
import { Loading } from './loading';
import { useSelf } from '../store';

export const Wrapper = (props: ParentProps<{ state: state.State }>) => {
  const self = useSelf();

  // TODO: Replace raw messages with error pages
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
        <ErrorBoundary
          fallback={error => {
            console.debug('INNER CAUGHT', error);
            if ('millis' in error) {
              return <TimeOut />;
            }

            return <GenericError />;
          }}
        >
          <Suspense fallback={<Loading />}>
            <Show when={self()?.pending !== true} fallback={<Welcome />}>
              {props.children}
            </Show>
          </Suspense>
        </ErrorBoundary>
      </Match>
    </Switch>
  );
};
