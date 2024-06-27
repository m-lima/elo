import { Suspense } from 'solid-js';

import { useGames, usePlayers } from '../store';
import { type Game, type Player } from '../types';
import { Games as GameTable } from '../components';
import { Loading } from '../page';

export const Games = () => {
  const games = useGames();
  const players = usePlayers();

  return <Suspense fallback={<Loading />}>{wrapRender(games(), players())}</Suspense>;
};

const wrapRender = (games: Game[] = [], players: Player[] = []) => (
  <GameTable players={players} games={games} />
);
