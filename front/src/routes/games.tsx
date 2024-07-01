import { Suspense } from 'solid-js';

import { useGames, usePlayers } from '../store';
import { type Game, type Player } from '../types';
import { icon, Action, Games as GameTable, Actions } from '../components';
import { Loading, Main } from '../pages';

export const Games = () => {
  const games = useGames();
  const players = usePlayers();

  return <Suspense fallback={<Loading />}>{wrapRender(games(), players())}</Suspense>;
};

const wrapRender = (games: Game[] = [], players: Player[] = []) => (
  <>
    <Actions>
      <Action
        icon={<icon.Add />}
        text='New game'
        action={() => {
          console.debug('Clicked');
        }}
      />
    </Actions>
    <Main>
      <GameTable players={players} games={games} />
    </Main>
  </>
);
