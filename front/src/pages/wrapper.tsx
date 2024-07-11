import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { TimeOut, GenericError, Unauthorized } from './error';
import { Welcome } from './welcome';
import { Loading } from './loading';
import { useStore } from '../store';

export const Wrapper = (props: ParentProps<{ state: state.State }>) => (
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
          console.debug(error);
          return 'millis' in error ? <TimeOut /> : <GenericError />;
        }}
      >
        <Suspense fallback=<Loading />>
          <InviteWrapper>{props.children}</InviteWrapper>
        </Suspense>
      </ErrorBoundary>
    </Match>
  </Switch>
);

const InviteWrapper = (props: ParentProps) => {
  const self = useStore().getSelf();

  return (
    <Show when={self()?.pending !== true} fallback=<Welcome />>
      {props.children}
    </Show>
  );
};
