export type User = {
  readonly id: number;
  readonly pending?: boolean;
};

export type Player = {
  readonly id: number;
  readonly name: string;
  readonly email: string;
  readonly inviter?: number;
  readonly createdMs: number;
};

export type PlayerTuple = [number, string, string, number | null, number];

export const playerFromTuple = ([id, name, email, inviter, createdMs]: PlayerTuple): Player => {
  return {
    id,
    name,
    email,
    inviter: inviter !== null ? inviter : undefined,
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
  readonly ratingDelta: number;
  readonly challenge: boolean;
  readonly deleted: boolean;
  readonly millis: number;
  readonly createdMs: number;
};

export type GameTuple = [
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  number,
  boolean,
  boolean,
  number,
  number,
];

export const gameFromTuple = ([
  id,
  playerOne,
  playerTwo,
  scoreOne,
  scoreTwo,
  ratingOne,
  ratingTwo,
  ratingDelta,
  challenge,
  deleted,
  millis,
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
    ratingDelta,
    challenge,
    deleted,
    millis,
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

export type EnrichedPlayer = Player & {
  readonly position: number;
  readonly rating: number;
  readonly games: number;
  readonly wins: number;
  readonly losses: number;
  readonly challengesWon: number;
  readonly challengesLost: number;
  readonly pointsWon: number;
  readonly pointsLost: number;
};

export type EnrichedGame = Game & {
  readonly playerOneName?: string;
  readonly playerTwoName?: string;
  readonly balanceOne: number;
  readonly balanceTwo: number;
};

export type Getter<T> = () => T | undefined;
