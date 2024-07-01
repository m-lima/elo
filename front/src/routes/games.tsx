import { Suspense } from 'solid-js';

import { Store, useGames, usePlayers, useStore } from '../store';
import { type Game, type Player } from '../types';
import { icon, Action, Games as GameTable, Actions } from '../components';
import { Loading, Main } from '../pages';

export const Games = () => {
  const store = useStore();
  const games = useGames(store);
  const players = usePlayers(store);

  console.debug('Outer', games);

  return (
    <>
      Outer: {games()?.length}
      <br />
      <Suspense fallback={<Loading />}>{wrapRender(store, games(), players())}</Suspense>
    </>
  );
};

const wrapRender = (store: Store, games: Game[] = [], players: Player[] = []) => {
  console.debug('Inner', games);
  return (
    <>
      Inner: {games.length}
      <br />
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
    </>
  );
};
