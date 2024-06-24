import { Socket, state } from '../socket';
import {
  type Player,
  type Game,
  type Invite,
  type User,
  playerFromTuple,
  gameFromTuple,
  inviteFromTuple,
} from '../types';
import { type Message, type Request } from './message';
import { newRequestId, validateMessage, validateMessages } from './request';

export class Store {
  private readonly socket: Socket<Request, Message>;

  readonly self: Resource<User>;
  readonly players: Resource<Player[]>;
  readonly games: Resource<Game[]>;
  readonly invites: Resource<Invite[]>;

  public static makeSocket(url: string | URL, checkUrl?: string | URL): Socket<Request, Message> {
    return new Socket(url, checkUrl);
  }

  public constructor(socket: Socket<Request, Message>) {
    this.socket = socket;

    this.socket.registerStateListener(newState => {
      if (newState === state.Connected.Open) {
        this.refresh();
      }
    });

    this.self = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { player: 'id' } }, message => {
        const validated = validateMessages(id, ['user', 'pending'], message);

        if (validated === undefined) {
          return;
        }

        if (validated.user !== undefined) {
          return { id: validated.user, pending: false };
        } else if (validated.pending !== undefined) {
          return { id: validated.pending, pending: true };
        }
      });
    });

    this.players = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { player: 'list' } }, message => {
        const validated = validateMessage(id, 'players', message);

        if (validated === undefined) {
          return;
        }

        return validated.map(playerFromTuple);
      });
    });

    this.games = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { game: 'list' } }, message => {
        const validated = validateMessage(id, 'games', message);

        if (validated === undefined) {
          return;
        }

        return validated.map(gameFromTuple);
      });
    });

    this.invites = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { invite: 'list' } }, message => {
        const validated = validateMessage(id, 'invites', message);

        if (validated === undefined) {
          return;
        }

        return validated.map(inviteFromTuple);
      });
    });
  }

  public async invitationRsvp(rsvp: boolean) {
    const id = newRequestId();
    await this.socket.request({ id, do: { invite: rsvp ? 'accept' : 'reject' } }, message => {
      const validated = validateMessage(id, 'done');
    });
  }

  public getSelf() {
    return this.self.get();
  }

  public getPlayers() {
    return this.players.get();
  }

  public getGames() {
    return this.games.get();
  }

  public getInvites() {
    return this.invites.get();
  }

  private refresh() {
    if (this.self.isPresent()) {
      void this.self.get();
    }

    if (this.players.isPresent()) {
      void this.players.get();
    }

    if (this.games.isPresent()) {
      void this.games.get();
    }

    if (this.invites.isPresent()) {
      void this.invites.get();
    }
  }
}

type Listener<T> = (data: T) => void;

class Resource<T> {
  private readonly fetcher: () => Promise<T>;

  private data?: T;
  private debouncer?: Promise<T>;
  private listeners: Listener<T>[];

  constructor(fetcher: () => Promise<T>) {
    this.fetcher = fetcher;

    this.listeners = [];
  }

  public isPresent(): boolean {
    return !!this.data;
  }

  public getRaw(): T | undefined {
    return this.data;
  }

  public get(): Promise<T> {
    if (this.data !== undefined) {
      return Promise.resolve(this.data);
    }

    if (!this.debouncer) {
      this.debouncer = this.fetcher()
        .then(data => {
          this.set(data);
          return data;
        })
        .finally(() => (this.debouncer = undefined));
    }

    return this.debouncer;
  }

  // TODO: Maybe do a deeper compare?
  // TODO: If doing deep compare, set only fields that don't match?
  public set(data: T) {
    if (this.debouncer !== undefined) {
      // TODO: Cancel debouncer from updating after setting
    }

    if (this.data !== data) {
      this.data = data;
      this.listeners.forEach(l => {
        l(data);
      });
    }
  }

  public registerListener(listener: Listener<T>): Listener<T> {
    this.listeners.push(listener);
    return listener;
  }

  public unregisterListener(listener: Listener<T>) {
    const index = this.listeners.indexOf(listener);
    if (index >= 0) {
      this.listeners.splice(index, 1);
    }
  }
}
