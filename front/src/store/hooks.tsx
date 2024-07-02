import { ParentProps, createContext, useContext } from 'solid-js';

import { Store } from './store';

const StoreContext = createContext<Store>();

export const WithStore = (props: ParentProps<{ store: Store }>) => (
  <StoreContext.Provider value={props.store}>{props.children}</StoreContext.Provider>
);

// Allowed because if misused, better to throw errors instead of paying for runtime checks
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
export const useStore = () => useContext(StoreContext)!;
