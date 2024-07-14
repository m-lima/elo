import { Suspense, createSignal } from 'solid-js';

import { useStore } from '../store';
import { prompt, Games as GameTable, action } from '../components';
import { Loading, Main } from '../pages';

export const Games = () => {
  const store = useStore();
  const self = store.useSelf();
  const games = store.useGames();
  const players = store.usePlayers();
  const [promptVisible, setPromptVisible] = createSignal(false);

  return (
    <Suspense fallback=<Loading />>
      <prompt.Game
        visible={promptVisible}
        hide={() => setPromptVisible(false)}
        store={store}
        self={() => players()?.find(p => p.id === self()?.id)}
        players={players}
      />
      <action.Actions>
        <action.Game action={() => setPromptVisible(true)} />
      </action.Actions>
      <Main>
        <GameTable players={players} games={games} />
      </Main>
    </Suspense>
  );
};
