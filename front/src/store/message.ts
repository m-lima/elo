import {
  type Game,
  type GameTuple,
  type HistoryTuple,
  type Invite,
  type InviteTuple,
  type Player,
  type PlayerTuple,
  type User,
} from '../types';

export type Ided = {
  id: number;
};

export type Request = Ided & { do: 'version' | RequestPlayer | RequestGame | RequestInvite };
export type RequestPlayer = { player: 'id' | 'list' | { rename: string } };
export type RequestGame = {
  game:
    | 'list'
    | {
        register: {
          player: number;
          opponent: number;
          score: number;
          opponentScore: number;
          challenge: boolean;
          millis: number;
        };
      }
    | { update: Game }
    | { history: number };
};
export type RequestInvite = {
  invite:
    | 'list'
    | { player: { name: string; email: string } }
    | { cancel: number }
    | 'accept'
    | 'reject';
};

export type Message = MessagePush | MessageOk | MessageError;
export type MessageOk = { id: number; ok: Ok };
export type MessageError = { id?: number; error: Error };
export type MessagePush = { push: Push };

export type Ok = 'done' | OkResponse;

export type OkResponse = {
  version: number;
  user: User;
  players: PlayerTuple[];
  games: GameTuple[];
  invites: InviteTuple[];
  history: HistoryTuple[];
};

export type Error = {
  code: number;
  message: string;
};

export type Push = { player: PushPlayer } | { game: PushGame };
export type PushPlayer =
  | { renamed: { player: number; old: string; new: string } }
  | { invited: Invite }
  | { uninvited: Invite }
  | { joined: Player };
export type PushGame = { registered: PushGamePayload } | { updated: PushGamePayload };
export type PushGamePayload = {
  game: number;
  updates: GameTuple[];
};
