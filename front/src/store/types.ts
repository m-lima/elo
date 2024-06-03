import type { User } from '../types';

export interface Backend {
  users: {
    self(): Promise<User>,
    list(): Promise<User[]>,
  },
}

export type Data = {
  self?: User;
  users?: User[];
};

export type Listeners = {
  self: Listener<User>[];
  users: Listener<User[]>[];
};

export type Debouncers = {
  users: {
    self?: Promise<User>,
    list?: Promise<User[]>,
  },
};

export type Listener<T> = (data: T) => void;
