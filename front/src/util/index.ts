import { type Player, type Game } from '../types';

export const name = 'EloPong';

export const monthToString = (month: number) => {
  switch (month) {
    case 0:
      return 'Jan';
    case 1:
      return 'Feb';
    case 2:
      return 'Mar';
    case 3:
      return 'Apr';
    case 4:
      return 'May';
    case 5:
      return 'Jun';
    case 6:
      return 'Jul';
    case 7:
      return 'Aug';
    case 8:
      return 'Sep';
    case 9:
      return 'Oct';
    case 10:
      return 'Nov';
    case 11:
      return 'Dec';
  }
};

export type Getter<T> = () => T | undefined;

export const compareLists = <T>(a: T[], b: T[]) => {
  if (a.length !== b.length) {
    return false;
  }

  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) {
      return false;
    }
  }

  return true;
};

export const sortPlayers = <T extends Pick<Player, 'rating' | 'createdMs'>>(a: T, b: T) => {
  const result = b.rating - a.rating;
  if (result !== 0) {
    return result;
  }

  return a.createdMs - b.createdMs;
};

export const enrichPlayers = (players: Player[] = [], games: Game[] = []) => {
  const enrichedPlayers = new Map<number, EnrichedPlayer>(
    players.map((p, i) => [
      p.id,
      {
        position: i + 1,
        games: 0,
        wins: 0,
        losses: 0,
        challengesWon: 0,
        challengesLost: 0,
        pointsWon: 0,
        pointsLost: 0,
        ...p,
      },
    ]),
  );

  for (const game of games) {
    const player_one = enrichedPlayers.get(game.playerOne);
    if (player_one !== undefined) {
      player_one.games += 1;
      player_one.wins += 1;
      player_one.pointsWon += game.scoreOne;
      player_one.pointsLost += game.scoreTwo;
      if (game.challenge) {
        player_one.challengesWon += 1;
      }
    }

    const player_two = enrichedPlayers.get(game.playerTwo);
    if (player_two !== undefined) {
      player_two.games += 1;
      player_two.losses += 1;
      player_two.pointsLost += game.scoreOne;
      player_two.pointsWon += game.scoreTwo;
      if (game.challenge) {
        player_two.challengesLost += 1;
      }
    }
  }

  return Array.from(enrichedPlayers.values()).sort(sortPlayers);
};

export type EnrichedPlayer = Player & {
  position: number;
  games: number;
  wins: number;
  losses: number;
  challengesWon: number;
  challengesLost: number;
  pointsWon: number;
  pointsLost: number;
};
