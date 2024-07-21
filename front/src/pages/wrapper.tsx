import { ErrorBoundary, Match, ParentProps, Show, Suspense, Switch } from 'solid-js';

import { state } from '../socket';
import { Welcome } from './welcome';
import { Loading } from './loading';
import { useStore } from '../store';
import * as error from './error';

export const Wrapper = (props: ParentProps<{ state: state.State }>) => (
  <Switch>
    <Match when={props.state === state.Disconnected.Connecting}>
      <Loading />
    </Match>
    <Match when={props.state === state.Disconnected.Unauthorized}>
      <error.Unauthorized />
    </Match>
    <Match when={state.isDisconnected(props.state)}>
      <error.GenericError />
    </Match>
    <Match when={true}>
      <ErrorBoundary
        fallback={error => ('millis' in error ? <error.TimeOut /> : <error.GenericError />)}
      >
        <InviteWrapper>
          <VersionWrapper>{props.children}</VersionWrapper>
        </InviteWrapper>
      </ErrorBoundary>
    </Match>
  </Switch>
);

const VersionWrapper = (props: ParentProps) => {
  const version = useStore().checkVersion();

  return (
    <Suspense fallback=<Loading />>
      <Show when={version() === true} fallback=<error.Version />>
        {props.children}
      </Show>
    </Suspense>
  );
};

const InviteWrapper = (props: ParentProps) => {
  const self = useStore().useSelf();

  return (
    <Suspense fallback=<Loading />>
      <Show when={self()?.pending !== true} fallback=<Welcome />>
        {props.children}
      </Show>
    </Suspense>
  );
};
