import { Socket } from '../socket';
import type { User } from '../types';
import { Backend } from './types';

export class Ws implements Backend {
  private readonly socket: Socket;

  public constructor(url: string | URL) {
    this.socket = new Socket(url);
  }

  public getSelf(): Promise<User> {
    throw new Error('Method not implemented.');
  }

  public getPlayers(): Promise<User[]> {
    throw new Error('Method not implemented.');
  }
}
