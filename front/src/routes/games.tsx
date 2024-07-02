import { Suspense } from 'solid-js';

import { useStore } from '../store';
import { icon, Action, Games as GameTable, Actions } from '../components';
import { Loading, Main } from '../pages';

export const Games = () => {
  const store = useStore();
  const games = store.getGames();
  const players = store.getPlayers();

  return (
    <Suspense fallback={<Loading />}>
      <Actions>
        <Action
          icon={<icon.Add />}
          text='New game'
          action={() => {
            void store.registerGame(7, 17, 21);
          }}
        />
      </Actions>
      <Main>
        <GameTable players={players} games={games} />
      </Main>
    </Suspense>
  );
};
