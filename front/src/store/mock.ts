/// EXCLUDE(PROD)

import { Backend } from './store';

export class Mock implements Backend {
  async info() {
    return {
      id: 27,
      name: 'My Name',
      email: 'email@domain.com',
      score: 2000,
      wins: 10,
      losses: 7,
      pointsWon: 10 * 11 + 7 * 5,
      pointsLost: 10 * 5 + 7 * 11,
      created: new Date(),
    };
  }

  async userList() {
    return [await this.info()];
  }
}
