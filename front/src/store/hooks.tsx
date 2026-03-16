import { ParentProps, createContext, useContext } from 'solid-js';

import { Store } from './store';

const StoreContext = createContext<Store>();

export const WithStore = (props: ParentProps<{ store: Store }>) => (
  <StoreContext.Provider value={props.store}>{props.children}</StoreContext.Provider>
);

export const useStore = () => {
  const context = useContext(StoreContext);
  if (context === undefined) {
    throw new Error('`useStore` must be used inside a <WithStore>');
  }
  return context;
};
