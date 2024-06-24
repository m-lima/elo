import { type InviteTuple, type GameTuple, type PlayerTuple } from '../types';

export type Ided = {
  id: number;
};

export type Request = Ided & { do: PlayerRequest | GameRequest | InviteRequest };
export type PlayerRequest = { player: 'id' | 'list' | { rename: string } };
export type GameRequest = { game: 'list' };
export type InviteRequest = { invite: 'list' | 'accept' | 'reject' };

export type Message = PushMessage | OkMessage | ErrorMessage;
export type OkMessage = { id: number; ok: Ok };
export type ErrorMessage = { id?: number; error: Error };
export type PushMessage = { push: string };

export type Ok = 'done' | OkResponse;

export type OkResponse = {
  user?: number;
  pending?: number;
  players?: PlayerTuple[];
  games?: GameTuple[];
  invites: InviteTuple[];
};

export type Error = {
  code: number;
  message?: string;
};
