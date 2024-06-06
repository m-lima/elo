import type { Player } from '../types';

export interface Backend {
  getSelf(): Promise<Player>;
  getPlayers(): Promise<Player[]>;
}

export type Listener<T> = (data: T) => void;
