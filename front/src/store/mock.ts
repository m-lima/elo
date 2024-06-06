/// EXCLUDE(PROD)

import { Player, byPosition } from '../types';
import { Backend } from './types';

export class Mock implements Backend {
  private getSelfCount = 0;
  private getPlayersCount = 0;

  public getSelf() {
    this.getSelfCount += 1;
    console.log(`Called self() ${this.getSelfCount} times`);
    return this.getPlayers().then(u => u.filter(u => u.id === 27)[0]);
  }

  public getPlayers() {
    const makePlayer = (id: number) => {
      const first = Math.floor(Math.random() * 10000);
      const last = Math.floor(Math.random() * 10000);

      const name = `${first} ${last}`;
      const email = `${first}.${last}@email.com`;

      const wins = Math.floor(Math.random() * 100);
      const losses = Math.floor(Math.random() * 100);

      const pointsWon = wins * 11 + Math.floor(Math.random() * losses * 7.5);
      const pointsLost = losses * 11 + Math.floor(Math.random() * wins * 7.5);

      return {
        id,
        name,
        email,
        position: 0,
        score: 2000,
        wins,
        losses,
        pointsWon,
        pointsLost,
        created: new Date(),
      };
    };

    const self = {
      id: 27,
      name: 'My Name',
      email: 'email@domain.com',
      position: 3,
      score: 2000,
      wins: 10,
      losses: 7,
      pointsWon: 10 * 11 + 7 * 5,
      pointsLost: 10 * 5 + 7 * 11,
      created: new Date(),
    };
    const players = Array(10)
      .fill(undefined)
      .map((_, i) => (i === 0 ? self : makePlayer(i)));
    players.sort(byPosition);

    this.getPlayersCount += 1;
    console.log(`Called list() ${this.getPlayersCount} times`);

    return new Promise<Player[]>((accept, _reject) => {
      setTimeout(() => {
        accept(players);
      }, 1000);
    });
  }
}
