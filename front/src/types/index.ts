// TODO: Move into types/index.ts

export type Player = {
  readonly id: number;
  readonly name: string;
  readonly email: string;
  readonly position: number;
  readonly score: number;
  readonly wins: number;
  readonly losses: number;
  readonly pointsWon: number;
  readonly pointsLost: number;
  readonly created: Date;
};

export const byPosition = (a: Player, b: Player) => {
  const position = a.position - b.position;
  if (position !== 0) {
    return position;
  }

  const score = b.score - a.score;
  if (score !== 0) {
    return score;
  }

  const wins = b.wins - a.wins;
  if (wins !== 0) {
    return wins;
  }

  const losses = a.losses - b.losses;
  if (losses !== 0) {
    return losses;
  }

  return a.created.getTime() - b.created.getTime();
};

// TODO: Use these
/* eslint-disable-next-line
@typescript-eslint/no-unused-vars
*/
type Create<T> = Omit<T, 'id' | 'created'>;
/* eslint-disable-next-line
@typescript-eslint/no-unused-vars
*/
type Edit<T, N extends keyof T> = Partial<Pick<T, N>>;

// type CreatePlayer = Partial<Pick<Player, 'name' | 'email'>>;
