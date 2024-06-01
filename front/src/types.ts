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

export interface UserI {
  readonly id: number,
  readonly name: string,
  readonly email: string,
  readonly score: number,
  readonly wins: number,
  readonly losses: number,
  readonly pointsWon: number,
  readonly pointsLost: number,
  readonly created: Date,
}

// type CreateUser = Partial<Pick<User, 'name' | 'email'>>;
type Create<T> = Omit<T, 'id' | 'created'>;
type Edit<T, N extends keyof T> = Partial<Pick<T, N>>;
