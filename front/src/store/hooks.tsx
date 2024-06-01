import { ParentProps, createContext, createEffect, createResource, onCleanup, useContext } from "solid-js";

import { Store } from "./store";

const StoreContext = createContext<Store>();

export const useStore = () => useContext(StoreContext)!;
export const WithStore = (props: ParentProps<{ store: Store }>) =>
  <StoreContext.Provider value={props.store}>
    {props.children}
  </StoreContext.Provider>

export const useSelf = (maybeStore?: Store) => {
  const store = maybeStore === undefined ? useStore() : maybeStore;
  const [self, { mutate }] = createResource(() => store.users.self());

  createEffect(() => {
    const listener = store.listener.register.self(mutate);
    onCleanup(() => store.listener.unregister.self(listener));
  });

  return self;
}

