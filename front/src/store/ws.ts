import { Socket } from '../socket';
import type { User } from '../types';
import { Backend } from './store';

export class Ws implements Backend {
  private readonly socket: Socket;

  public constructor(url: string | URL) {
    this.socket = new Socket(url);
  }

  readonly users = {
    self(): Promise<User> {
      throw new Error('Method not implemented.');
    },

    list(): Promise<User[]> {
      throw new Error('Method not implemented.');
    },
  };
}
