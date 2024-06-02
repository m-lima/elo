import { ParentProps, createContext, createEffect, createResource, onCleanup, useContext } from 'solid-js';

import { Store } from './store';
import { User } from '../types';

const StoreContext = createContext<Store>();

export const WithStore = (props: ParentProps<{ store: Store }>) =>
  <StoreContext.Provider value={props.store}>
    {props.children}
  </StoreContext.Provider>;

export const useStore = () => useContext(StoreContext)!;

const UserContext = createContext<User>();

export const WithSelf = (props: ParentProps<{ self: User }>) =>
  <UserContext.Provider value={props.self}>
    {props.children}
  </UserContext.Provider>;

export const useAsyncSelf = (store: Store) => {
  const [self, { mutate }] = createResource(() => store.users.self());

  createEffect(() => {
    const listener = store.listener.register.self(mutate);
    onCleanup(() => store.listener.unregister.self(listener));
  });

  return self;
};

export const useSelf = () => useContext(UserContext)!;
