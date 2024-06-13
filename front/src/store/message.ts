import { Player } from '../types';

export type Ided = {
  id: number;
};

export type Request = Ided & { do: PlayerRequest };
export type PlayerRequest = { player: 'id' | 'list' | { rename: string } };

export type Message = PushMessage | OkMessage | ErrorMessage;
export type OkMessage = { id: number; ok: Ok };
export type ErrorMessage = { id?: number; error: Error };
export type PushMessage = { push: string };

export type Ok = { id?: number; players?: Player[] };

export type Error = {
  code: number;
  message?: string;
};
