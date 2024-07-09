import { Show, Suspense, createMemo, createSignal } from 'solid-js';

import { useStore } from '../store';
import { prompt, Games as GameTable, action } from '../components';
import { Loading, Main } from '../pages';
import { buildOpponentList } from '../util';

export const Games = () => {
  const store = useStore();
  const self = store.getSelf();
  const games = store.getGames();
  const players = store.getPlayers();
  const [promptVisible, setPromptVisible] = createSignal(false);
  const shown = createMemo(prev => {
    if (prev === true) {
      return true;
    } else {
      return promptVisible();
    }
  });
  const opponents = createMemo(() => {
    if (!shown()) {
      return [];
    }

    const innerSelf = self()?.id;
    const innerGames = games();
    const innerPlayers = players();

    if (innerSelf === undefined || innerGames === undefined || innerPlayers === undefined) {
      return [];
    }

    return buildOpponentList(innerGames, innerPlayers, innerSelf);
  });

  return (
    <Suspense fallback=<Loading />>
      <Show when={promptVisible()}>
        <prompt.Game
          hide={() => setPromptVisible(false)}
          store={store}
          self={() => players()?.find(p => p.id === self()?.id)}
          opponents={opponents}
        />
      </Show>
      <action.Actions>
        <action.Game action={() => setPromptVisible(true)} />
      </action.Actions>
      <Main>
        <GameTable players={players} games={games} />
      </Main>
    </Suspense>
  );
};
