import type { User } from '../types';
import { Mock } from './mock';

export class Store implements Backend {
  private readonly backend: Backend;

  private readonly data: Data;

  private readonly promises: Promises;
  private readonly listeners: Listeners;

  public constructor(url?: string | URL) {
    if (!!url) {
      throw new Error('Remote backend not implemented');
    }
    this.backend = new Mock();

    this.data = {};

    this.promises = {
      users: {},
    };

    this.listeners = {
      self: [],
      users: [],
    };
  }

  readonly users = {
    self: async () => {
      const self = this.data.self;
      if (!!self) {
        return self;
      }

      if (!this.promises.users.self) {
        this.promises.users.self = this.backend.users.self()
          .then(self => {
            this.data.self = self;
            return self;
          })
          .finally(() => this.promises.users.self = undefined);
      }

      return await this.promises.users.self;
    },

    list: async () => {
      const list = this.data.users;
      if (!!list) {
        return list;
      }

      if (!this.promises.users.list) {
        this.promises.users.list = this.backend.users.list()
          .then(users => {
            this.data.users = users;
            return users;
          })
          .finally(() => this.promises.users.list = undefined);
      }

      return await this.promises.users.list;
    },
  };

  readonly listener = {
    register: {
      self: (handler: (data: User) => void): Listener<User> => {
        const listener = new Listener(handler);
        this.listeners.self.push(listener);
        return listener;
      },
      users: (handler: (data: User[]) => void): Listener<User[]> => {
        const listener = new Listener(handler);
        this.listeners.users.push(listener);
        return listener;
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
}

export interface Backend {
  users: {
    self(): Promise<User>,
    list(): Promise<User[]>,
  },
}

type Data = {
  self?: User;
  users?: User[];
};

type Listeners = {
  self: Listener<User>[];
  users: Listener<User[]>[];
};

type Promises = {
  users: {
    self?: Promise<User>,
    list?: Promise<User[]>,
  },
};

class Listener<T> {
  readonly handler: (data: T) => void;

  constructor(handler: (data: T) => void) {
    this.handler = handler;
  }
}
