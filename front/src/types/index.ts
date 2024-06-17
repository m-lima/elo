// TODO: Move into types/index.ts

export type Player = {
  readonly id: number;
  readonly name: string;
  readonly email: string;
  readonly inviter?: number;
  readonly rating: number;
  readonly createdMs: number;
};

export type PlayerTuple = [number, string, string, number | null, number, number];

export const playerFromTuple = ([
  id,
  name,
  email,
  inviter,
  rating,
  createdMs,
]: PlayerTuple): Player => {
  return {
    id,
    name,
    email,
    inviter: inviter !== null ? inviter : undefined,
    rating,
    createdMs,
  };
};

export type Game = {
  readonly id: number;
  readonly playerOne: number;
  readonly playerTwo: number;
  readonly scoreOne: number;
  readonly scoreTwo: number;
  readonly createdMs: number;
};

export type GameTuple = [number, number, number, number, number, number, number, number];

export const gameFromTuple = ([
  id,
  playerOne,
  playerTwo,
  scoreOne,
  scoreTwo,
  createdMs,
]: GameTuple): Game => {
  return {
    id,
    playerOne,
    playerTwo,
    scoreOne,
    scoreTwo,
    createdMs,
  };
};

// TODO: Use these
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Create<T> = Omit<T, 'id' | 'created'>;
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Edit<T, N extends keyof T> = Partial<Pick<T, N>>;

// type CreatePlayer = Partial<Pick<Player, 'name' | 'email'>>;
