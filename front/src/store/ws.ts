import { Socket } from '../socket';
import type { Player } from '../types';
import { Backend } from './types';

export class Ws implements Backend {
  private readonly socket: Socket;

  public constructor(url: string | URL) {
    this.socket = new Socket(url);
  }

  public getSelf(): Promise<Player> {
    throw new Error('Method not implemented.');
  }

  public getPlayers(): Promise<Player[]> {
    throw new Error('Method not implemented.');
  }
}
