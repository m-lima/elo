/// EXCLUDE(PROD)

import { User } from '../types';
import { Backend } from './store';

export class Mock implements Backend {
  private count = 0;

  readonly users = {
    self: async () => {
      this.count += 1;
      console.log(`Called self() ${this.count} times`);
      return new Promise<User>(accept => {
        setTimeout(() => {
          accept({
            id: 27,
            name: 'My Name',
            email: 'email@domain.com',
            score: 2000,
            wins: 10,
            losses: 7,
            pointsWon: 10 * 11 + 7 * 5,
            pointsLost: 10 * 5 + 7 * 11,
            created: new Date(),
          })
        }, 1000);
      });
    },

    list: async () => {
      return [await this.users.self()];
    },
  };
}
