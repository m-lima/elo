/// EXCLUDE(PROD)

import { User, byPosition } from '../types';
import { Backend } from './types';

export class Mock implements Backend {
  private selfCount = 0;
  private listCount = 0;

  public getSelf() {
    this.selfCount += 1;
    console.log(`Called self() ${this.selfCount} times`);
    return this.getUsers().then(u => u.filter(u => u.id === 27)[0]);
  }

  public getUsers() {
    const makeUser = (id: number) => {
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

    let self = {
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
    let users = Array(10).fill(undefined).map((_, i) => i === 0 ? self : makeUser(i));
    users.sort(byPosition);

    this.listCount += 1;
    console.log(`Called list() ${this.listCount} times`);

    return new Promise<User[]>((accept, reject) => {
      setTimeout(() => accept(users), 1000);
    });
  }
}
