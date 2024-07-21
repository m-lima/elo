import { Show, Suspense, createSignal } from 'solid-js';

import { useStore } from '../store';
import { prompt, Games as GameTable, action } from '../components';
import { error, Loading, Main } from '../pages';

export const Games = () => {
  const store = useStore();
  const self = store.useSelf();
  const players = store.usePlayers();
  const games = store.useEnrichedGames();
  const [promptVisible, setPromptVisible] = createSignal(false);

  return (
    <Suspense fallback=<Loading />>
      <prompt.Game
        visible={promptVisible}
        hide={() => setPromptVisible(false)}
        store={store}
        players={players}
        self={() => players()?.find(p => p.id === self()?.id)}
      />
      <action.Actions>
        <action.Game action={() => setPromptVisible(true)} />
      </action.Actions>
      <Main>
        <Show when={games().length > 0} fallback=<error.NoGames inline />>
          <GameTable games={games} />
        </Show>
      </Main>
    </Suspense>
  );
};
