import {
  Component,
  createContext,
  createEffect,
  createSignal,
  JSXElement,
  Show,
  useContext,
} from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { Requests } from "./utils/requests";

/** Context handle for global store. */
const StoreContext = createContext();

/** Provides a global store for state management.*/
export const StoreProdiver: Component<{ children: JSXElement }> = (props) => {
  const [store, setStore] = createStore();
  const [hasMounted, setHasMounted] = createSignal(false);

  // Runs after the initial render phase. Equivalent to onMount.
  createEffect(async () => {
    const response = (await Requests.getJson("/api/start")) as Store;
    setStore(response);
    setHasMounted(true);
  });

  // Render children when the store has been initialized.
  return (
    <StoreContext.Provider value={{ store, setStore }}>
      <Show when={hasMounted()}>{props.children}</Show>
    </StoreContext.Provider>
  );
};

/** Hook to access the global store. */
export function useStore() {
  return useContext(StoreContext) as StoreContext;
}

/** Store type. */
export type Store = {
  isAuthenticated: boolean;
  user: { username: string } | null;
};

/** Global store type. */
export type StoreContext = {
  store: Store;
  setStore: SetStoreFunction<Store>;
};
