import { usePlayers } from '../store';

export const Leaderboard = () => {
  const players = usePlayers();

  return <>{players.length}</>;
};
