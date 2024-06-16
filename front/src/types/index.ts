// TODO: Move into types/index.ts

export type Player = {
  readonly id: number;
  readonly name: string;
  readonly email: string;
  readonly inviter?: number;
  readonly rating: number;
  readonly createdMs: number;
};

export type Game = {
  readonly id: number;
  readonly playerOne: number;
  readonly playerTwo: number;
  readonly scoreOne: number;
  readonly scoreTwo: number;
  readonly createdMs: number;
};

// TODO: Use these
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Create<T> = Omit<T, 'id' | 'created'>;
// eslint-disable-next-line @typescript-eslint/no-unused-vars
type Edit<T, N extends keyof T> = Partial<Pick<T, N>>;

// type CreatePlayer = Partial<Pick<Player, 'name' | 'email'>>;
