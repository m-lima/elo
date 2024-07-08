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

export type PlayerStats = {
  wins: number;
  losses: number;
  pointsWon: number;
  pointsLost: number;
};

export type Game = {
  readonly id: number;
  readonly playerOne: number;
  readonly playerTwo: number;
  readonly scoreOne: number;
  readonly scoreTwo: number;
  readonly ratingOne: number;
  readonly ratingTwo: number;
  readonly challenge: boolean;
  readonly createdMs: number;
};

export type GameTuple = [number, number, number, number, number, number, number, boolean, number];

export const gameFromTuple = ([
  id,
  playerOne,
  playerTwo,
  scoreOne,
  scoreTwo,
  ratingOne,
  ratingTwo,
  challenge,
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
    challenge,
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
