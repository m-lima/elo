import {
  ParentProps,
  createContext,
  createEffect,
  createResource,
  onCleanup,
  useContext,
} from 'solid-js';

import { Store } from './store';
import { User } from '../types';

const StoreContext = createContext<Store>();

export const WithStore = (props: ParentProps<{ store: Store }>) => (
  <StoreContext.Provider value={props.store}>{props.children}</StoreContext.Provider>
);

export const useStore = () => useContext(StoreContext)!;

const UserContext = createContext<User>();

export const WithSelf = (props: ParentProps<{ self: User }>) => (
  <UserContext.Provider value={props.self}>{props.children}</UserContext.Provider>
);

export const useAsyncSelf = (store: Store) => {
  const [self, { mutate }] = createResource(() => store.self.get());

  createEffect(() => {
    const listener = store.self.registerListener(mutate);
    onCleanup(() => store.self.unregisterListener(listener));
  });

  return self;
};

export const useSelf = () => useContext(UserContext)!;

export const usePlayers = (maybeStore?: Store) => {
  const store = !maybeStore ? useStore() : maybeStore;
  const [players, { mutate }] = createResource(() => store.players.get());

  createEffect(() => {
    const listener = store.players.registerListener(mutate);
    onCleanup(() => store.players.unregisterListener(listener));
  });

  return players;
};
