import type { User } from '../types';

export interface Backend {
  getSelf(): Promise<User>,
  getPlayers(): Promise<User[]>,
}

export type Listener<T> = (data: T) => void;
