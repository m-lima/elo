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
import { type Message, type Request, type Ided } from './message';
import { FetchError, newRequestId, validateDone, validateMessage } from './request';

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

    this.socket.registerHandler(message => {
      if ('push' in message) {
        if ('player' in message.push) {
          if ('renamed' in message.push.player) {
            const rename = message.push.player.renamed;

            this.players.set(players =>
              players.map(p => (p.id === rename.player ? { ...p, name: rename.name } : p)),
            );
          } else if ('invited' in message.push.player) {
            const invite = message.push.player.invited;
            this.invites.set(invites => upsert(invites, invite));
          } else if ('uninvited' in message.push.player) {
            const uninvite = message.push.player.uninvited;
            this.invites.set(invites => invites.filter(i => i.id !== uninvite));
          } else if ('joined' in message.push.player) {
            const join = message.push.player.joined;
            this.invites.set(invites => invites.filter(i => i.email !== join.email));
            this.players.set(players => upsert(players, join));
          }
        } else if ('game' in message.push) {
          if ('registered' in message.push.game) {
            console.debug('Game registering', this.games.data?.length);
            const [game, playerOne, playerTwo] = message.push.game.registered;
            console.debug(JSON.stringify([game, playerOne, playerTwo]));
            this.games.set(games => upsert(games, game));
            console.debug('Game registered', this.games.data?.length);
            this.players.set(players => {
              upsert(players, playerOne);
              upsert(players, playerTwo);
              return players;
            });
          }
        }
      }
      return true;
    });

    this.self = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { player: 'id' } }, message => {
        const validated = validateMessage(id, ['user', 'pending'], message);

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

        return validated.players.map(playerFromTuple);
      });
    });

    this.games = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { game: 'list' } }, message => {
        const validated = validateMessage(id, 'games', message);

        if (validated === undefined) {
          return;
        }

        return validated.games.map(gameFromTuple);
      });
    });

    this.invites = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { invite: 'list' } }, message => {
        const validated = validateMessage(id, 'invites', message);

        if (validated === undefined) {
          return;
        }

        return validated.invites.map(inviteFromTuple);
      });
    });
  }

  public async renamePlayer(name: string) {
    const id = newRequestId();
    return this.socket.request({ id, do: { player: { rename: name } } }, message =>
      validateDone(id, message),
    );
  }

  public async invitePlayer(name: string, email: string) {
    const id = newRequestId();
    return this.socket.request({ id, do: { invite: { player: { name, email } } } }, message =>
      validateDone(id, message),
    );
  }

  public async cancelInvitattion(cancel: number) {
    const id = newRequestId();
    return this.socket.request({ id, do: { invite: { cancel } } }, message =>
      validateDone(id, message),
    );
  }

  public async invitationRsvp(rsvp: boolean) {
    const id = newRequestId();
    await this.socket.request({ id, do: { invite: rsvp ? 'accept' : 'reject' } }, message =>
      validateDone(id, message),
    );
  }

  public async registerGame(opponent: number, score: number, opponentScore: number) {
    const id = newRequestId();
    await this.socket.request(
      { id, do: { game: { register: { opponent, score, opponentScore } } } },
      message => validateDone(id, message),
    );
  }

  private refresh() {
    if (this.self.isPresent()) {
      void this.self.get(true);
    }

    if (this.players.isPresent()) {
      void this.players.get(true);
    }

    if (this.games.isPresent()) {
      void this.games.get(true);
    }

    if (this.invites.isPresent()) {
      void this.invites.get(true);
    }
  }
}

type Listener<T> = (data: T) => void;

class Resource<T> {
  private readonly fetcher: () => Promise<T>;

  data?: T;
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

  public get(forceUpdate: boolean = false): Promise<T> {
    if (!forceUpdate && this.data !== undefined) {
      return Promise.resolve(this.data);
    }

    if (this.debouncer === undefined) {
      this.debouncer = this.fetcher()
        .then(data => {
          this.replace(data);
          return data;
        })
        .finally(() => (this.debouncer = undefined));
    }

    return this.debouncer;
  }

  public set(setter: (current: T) => T) {
    if (this.data === undefined) {
      return;
    }

    // TODO: Cancel debouncer from updating after setting
    // if (this.debouncer !== undefined) {
    // }

    this.replace(setter(this.data));
  }

  // TODO: Maybe do a deeper compare?
  // TODO: If doing deep compare, set only fields that don't match?
  private replace(data: T) {
    this.data = data;
    console.debug('Notifying listeners');
    this.listeners.forEach(l => {
      l(data);
    });
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

const upsert = <T extends Ided>(data: T[], datum: T) => {
  const idx = data.findIndex(d => d.id === datum.id);
  if (idx < 0) {
    data.push(datum);
  } else {
    data[idx] = datum;
  }
  return data;
};
