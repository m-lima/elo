/* @refresh reload */
import { render } from 'solid-js/web';
import { ErrorBoundary, ParentProps, Suspense } from 'solid-js';

import { Loading, Router } from './router';
import { Side } from './components';
import { Store, WithSelf, WithStore, useAsyncSelf } from './store';

import './index.css';

const store = new Store();
const root = document.getElementById('root');

const andThen = <T, R>(f: (value: T) => R, value?: T) => value && f(value);

const App = (props: ParentProps) => {
  const self = useAsyncSelf(store);

  return (
    <Suspense fallback={<Loading />}>
      {andThen(
        self => (
          <WithSelf self={self}>
            <Side />
            <div>{props.children}</div>
          </WithSelf>
        ),
        self(),
      )}
    </Suspense>
  );
};

// TODO: Better global fallback
render(
  () => (
    <WithStore store={store}>
      <ErrorBoundary
        fallback={error => {
          console.log('CAUGHT', error);
          return <h1>{JSON.stringify(error)}</h1>;
        }}
      >
        <Router root={App} />
      </ErrorBoundary>
    </WithStore>
  ),
  // Allowed because this is normal solid construct
  /* eslint-disable-next-line
     @typescript-eslint/no-non-null-assertion
  */
  root!,
);
