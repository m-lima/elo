import type { User } from '../types';

import { Mock } from './mock';
import type { Backend, Listener } from './types';

export class Store implements Backend {
  private readonly backend: Backend;

  readonly self: Resource<User>;
  readonly players: Resource<User[]>;

  public constructor(url?: string | URL) {
    if (!!url) {
      throw new Error('Remote backend not implemented');
    }
    this.backend = new Mock();

    this.self = new Resource(this.backend.getSelf);
    this.players = new Resource(this.backend.getPlayers);
  }

  public getSelf() {
    return this.self.get();
  }

  public getPlayers() {
    return this.players.get();
  }

  // TODO: This is just a cheeky test
  public increment() {
    const self = this.self.getRaw();
    if (self) {
      this.self.set({ ...self, id: self.id + 1 });
    }
  }

  private refresh() {
    if (this.self.isPresent()) {
      this.self.get();
    }

    if (this.players.isPresent()) {
      this.players.get();
    }
  }
}

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
    if (!!this.data) {
      this.debouncer = undefined;
      return Promise.resolve(this.data);
    }

    if (!this.debouncer) {
      this.debouncer = this.fetcher()
        .then(data => {
          this.set(data);
          return data;
        })
        .finally(() => this.debouncer = undefined);
    }

    return this.debouncer;
  }

  // TODO: Maybe do a deeper compare?
  // TODO: If doing deep compare, set only fields that don't match?
  public set(data: T) {
    if (!!this.debouncer) {
      // TODO: Cancel debouncer from updating after setting
    }

    if (this.data !== data) {
      this.data = data;
      this.listeners.forEach(l => l(data));
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
