import { createStore, SetStoreFunction, Store as SolidStore } from 'solid-js/store';

import type { User } from '../types';
import { Mock } from './mock';

export class Store implements Backend {
  private readonly backend: Backend;

  private readonly data: SolidStore<Data>;
  private readonly setData: SetStoreFunction<Data>;

  public constructor(url: string | URL | undefined) {
    if (!!url) {
      throw new Error("Remote backend not implemented");
    }
    this.backend = new Mock();

    let [data, setData] = createStore({});
    this.data = data;
    this.setData = setData;
  }

  async info() {
    var self = this.data.self;
    if (self === undefined) {
      self = await this.backend.info();
      this.setData('self', self);
    }
    return self;
  }

  async userList() {
    var users = this.data.users;
    if (users === undefined) {
      users = await this.backend.userList();
      this.setData('users', users);
    }
    return users;
  }
}

export interface Backend {
  info(): Promise<User>,
  userList(): Promise<User[]>,
}

type Data = {
  self?: User;
  users?: User[];
};
