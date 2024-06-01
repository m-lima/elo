export type User = {
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
}

export const byPosition = (a: User, b: User) => {
  const position = a.position - b.position;
  if (position !== 0) {
    return position;
  }

  const wins = b.position - a.position;
  if (wins !== 0) {
    return wins;
  }

  const losses = a.losses - b.losses;
  if (losses !== 0) {
    return losses;
  }

  return a.created.getTime() - b.created.getTime();
};

// type CreateUser = Partial<Pick<User, 'name' | 'email'>>;
type Create<T> = Omit<T, 'id' | 'created'>;
type Edit<T, N extends keyof T> = Partial<Pick<T, N>>;
