import type { User } from '../types';

import { Mock } from './mock';
import type { Backend, Data, Debouncers, Listener, Listeners } from './types';

export class Store implements Backend {
  private readonly backend: Backend;

  private readonly data: Data;

  private readonly debouncers: Debouncers;
  private readonly listeners: Listeners;

  public constructor(url?: string | URL) {
    if (!!url) {
      throw new Error('Remote backend not implemented');
    }
    this.backend = new Mock();

    this.data = {};

    this.debouncers = {
      users: {},
    };

    this.listeners = {
      self: [],
      users: [],
    };
  }

  public readonly users = {
    self: async () => {
      const self = this.data.self;
      if (!!self) {
        return self;
      }

      if (!this.debouncers.users.self) {
        this.debouncers.users.self = this.backend.users.self()
          .then(self => {
            this.data.self = self;
            return self;
          })
          .finally(() => this.debouncers.users.self = undefined);
      }

      return await this.debouncers.users.self;
    },

    list: async () => {
      const list = this.data.users;
      if (!!list) {
        return list;
      }

      if (!this.debouncers.users.list) {
        this.debouncers.users.list = this.backend.users.list()
          .then(users => {
            this.data.users = users;
            return users;
          })
          .finally(() => this.debouncers.users.list = undefined);
      }

      return await this.debouncers.users.list;
    },
  };

  public readonly listener = {
    register: {
      self: (handler: Listener<User>): Listener<User> => {
        this.listeners.self.push(handler);
        return handler;
      },
      users: (handler: Listener<User[]>): Listener<User[]> => {
        this.listeners.users.push(handler);
        return handler;
      },
    },
    unregister: {
      self: (listener: Listener<User>) => {
        const index = this.listeners.self.indexOf(listener);
        if (index >= 0) {
          this.listeners.self.splice(index, 1);
        }
      },
      users: (listener: Listener<User[]>) => {
        const index = this.listeners.users.indexOf(listener);
        if (index >= 0) {
          this.listeners.self.splice(index, 1);
        }
      },
    },
  };

  // TODO: This is just a cheeky test
  public increment() {
    if (!this.data.self) {
      return;
    }

    const self = { ...this.data.self, id: this.data.self.id + 1 };
    this.setSelf(self);
  }

  private setSelf(self: User) {
    this.data.self = self;
    this.listeners.self.forEach(l => l(self));
  }
}
