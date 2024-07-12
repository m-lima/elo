import { type Player } from '../types';

export const name = 'EloPong';

export const colors = {
  accent: '#ffa500',
};

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

export const sortPlayers = <T extends Pick<Player, 'rating' | 'createdMs'>>(a: T, b: T) => {
  const result = b.rating - a.rating;
  if (result !== 0) {
    return result;
  }

  return a.createdMs - b.createdMs;
};
