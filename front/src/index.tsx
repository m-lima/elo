/* @refresh reload */
import { render } from 'solid-js/web';
import { ErrorBoundary, ParentProps } from 'solid-js';

import { Router } from './router';
import { status, Side } from './components';
import { Store, WithStore } from './store';

import './index.css';

const store = new Store();
const root = document.getElementById('root');

const App = (props: ParentProps) => {
  return (
    <>
      <Side />
      <div>{props.children}</div>
    </>
  );
};

// TODO: Better global fallback
render(
  () => (
    <WithStore store={store}>
      <status.Loading />
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
