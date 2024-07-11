import {
  type InviteTuple,
  type GameTuple,
  type PlayerTuple,
  type Player,
  type Game,
  type Invite,
} from '../types';

export type Ided = {
  id: number;
};

export type Request = Ided & { do: RequestPlayer | RequestGame | RequestInvite };
export type RequestPlayer = { player: 'id' | 'list' | { rename: string } };
export type RequestGame = {
  game:
    | 'list'
    | {
        register: {
          opponent: number;
          score: number;
          opponentScore: number;
          challenge: boolean;
        };
      };
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
  user: { id: number; pending: boolean };
  players: PlayerTuple[];
  games: GameTuple[];
  invites: InviteTuple[];
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
export type PushGame = {
  registered: [Game, Player, Player];
};
