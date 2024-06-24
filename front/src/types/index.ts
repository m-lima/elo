// TODO: Move into types/index.ts

export type User = {
  id: number;
  pending: boolean;
};

export type Player = {
  readonly id: number;
  readonly name: string;
  readonly email: string;
  readonly inviter?: number;
  readonly rating: number;
  readonly wins: number;
  readonly losses: number;
  readonly pointsWon: number;
  readonly pointsLost: number;
  readonly createdMs: number;
};

export type PlayerTuple = [
  number,
  string,
  string,
  number | null,
  number,
  number,
  number,
  number,
  number,
  number,
];

export const playerFromTuple = ([
  id,
  name,
  email,
  inviter,
  rating,
  wins,
  losses,
  pointsWon,
  pointsLost,
  createdMs,
]: PlayerTuple): Player => {
  return {
    id,
    name,
    email,
    inviter: inviter !== null ? inviter : undefined,
    rating,
    wins,
    losses,
    pointsWon,
    pointsLost,
    createdMs,
  };
};

export type Game = {
  readonly id: number;
  readonly playerOne: number;
  readonly playerTwo: number;
  readonly scoreOne: number;
  readonly scoreTwo: number;
  readonly ratingOne: number;
  readonly ratingTwo: number;
  readonly createdMs: number;
};

export type GameTuple = [number, number, number, number, number, number, number, number];

export const gameFromTuple = ([
  id,
  playerOne,
  playerTwo,
  scoreOne,
  scoreTwo,
  ratingOne,
  ratingTwo,
  createdMs,
]: GameTuple): Game => {
  return {
    id,
    playerOne,
    playerTwo,
    scoreOne,
    scoreTwo,
    ratingOne,
    ratingTwo,
    createdMs,
  };
};

export type Invite = {
  readonly id: number;
  readonly inviter: number;
  readonly name: string;
  readonly email: string;
  readonly createdMs: number;
};

export type InviteTuple = [number, number, string, string, number];

export const inviteFromTuple = ([id, inviter, name, email, createdMs]: InviteTuple): Invite => {
  return {
    id,
    inviter,
    name,
    email,
    createdMs,
  };
};

// TODO: Use these
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Create<T> = Omit<T, 'id' | 'created'>;
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Edit<T, N extends keyof T> = Partial<Pick<T, N>>;

// type CreatePlayer = Partial<Pick<Player, 'name' | 'email'>>;
