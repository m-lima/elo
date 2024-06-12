import type { Player } from '../types';

export interface Backend {
  getSelf(): Promise<Player>;
  getPlayers(): Promise<Player[]>;
}

export type Listener<T> = (data: T) => void;

export type Ided = {
  id: number;
};

export type Request = Ided & { do: UserRequest };
export type UserRequest = 'info' | 'list' | { rename: string };
