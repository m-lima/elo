import { Socket, state } from '../socket';
import { type Player, type Game } from '../types';
import { type Message, type Request } from './message';
import { newRequestId, validateMessage } from './request';

export class Store {
  private readonly socket: Socket<Request, Message>;

  readonly self: Resource<number>;
  readonly players: Resource<Player[]>;
  readonly games: Resource<Game[]>;

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
        const validated = validateMessage(id, 'id', message);

        if (validated === undefined) {
          return;
        }

        return validated;
      });
    });

    this.players = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { player: 'list' } }, message => {
        const validated = validateMessage(id, 'players', message);

        if (validated === undefined) {
          return;
        }

        return validated;
      });
    });

    this.games = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { game: 'list' } }, message => {
        const validated = validateMessage(id, 'games', message);

        if (validated === undefined) {
          return;
        }

        return validated;
      });
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

  private refresh() {
    if (this.self.isPresent()) {
      void this.self.get();
    }

    if (this.players.isPresent()) {
      void this.players.get();
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
