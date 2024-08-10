import {
  createMemo,
  createResource,
  createSignal,
  ResourceActions,
  Resource as SolidResource,
} from 'solid-js';

import { Socket, state } from '../socket';
import {
  gameFromTuple,
  historyFromTuple,
  inviteFromTuple,
  playerFromTuple,
  type EnrichedGame,
  type EnrichedPlayer,
  type Game,
  type Invite,
  type Player,
  type User,
} from '../types';
import { type Message, type Request, type Ided, type PushPlayer, type PushGame } from './message';
import { newRequestId, ResponseError, validateDone, validateMessage } from './request';
import * as consts from '../consts';

export class Store {
  private readonly socket: Socket<Request, Message>;

  private readonly subscribers: Subscriber[];

  private readonly self: Resource<User>;
  private readonly players: Resource<Player[]>;
  private readonly games: Resource<Game[]>;
  private readonly invites: Resource<Invite[]>;

  private dataVersion?: number;

  public static makeSocket(
    url: string | URL,
    checkUrl?: string | URL,
    loginUrl?: string | URL,
  ): Socket<Request, Message> {
    return new Socket(url, checkUrl, loginUrl);
  }

  public constructor(socket: Socket<Request, Message>) {
    this.socket = socket;

    this.socket.registerStateListener(newState => {
      if (newState === state.Connected.Open) {
        this.refresh();
      }
    });

    this.socket.registerHandler(message => {
      if ('push' in message) {
        if ('player' in message.push) {
          this.handlePushPlayer(message.push.player);
        } else if ('game' in message.push) {
          this.handlePushGame(message.push.game);
        }
      }
      return true;
    });

    this.subscribers = [];

    this.self = new Resource(() => {
      const id = newRequestId();
      return this.socket.request(
        { id, do: { player: 'id' } },
        message => validateMessage(id, 'user', message)?.user,
      );
    });

    this.players = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { player: 'list' } }, message =>
        validateMessage(id, 'players', message)?.players.map(playerFromTuple),
      );
    });

    this.games = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { game: 'list' } }, message =>
        validateMessage(id, 'games', message)?.games.map(gameFromTuple),
      );
    });

    this.invites = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { invite: 'list' } }, message =>
        validateMessage(id, 'invites', message)?.invites.map(inviteFromTuple),
      );
    });
  }

  public checkVersion() {
    const id = newRequestId();
    const [version, _] = createResource(() =>
      this.socket.request({ id, do: 'version' }, message => {
        const version = validateMessage(id, 'version', message)?.version;

        if (version === undefined) {
          return;
        }

        if (version.server !== consts.version) {
          return false;
        }

        if (this.dataVersion === undefined) {
          this.dataVersion = version.data;
        } else {
          if (version.data !== this.dataVersion) {
            this.refresh();
          }
        }

        return true;
      }),
    );
    return version;
  }

  public useSelf() {
    return this.self.get();
  }

  public usePlayers() {
    return this.players.get();
  }

  public useGames() {
    return this.games.get();
  }

  public useInvites() {
    return this.invites.get();
  }

  public useEnrichedPlayers() {
    const players = this.players.get();
    const games = this.games.get();
    return createMemo(() => enrichPlayers(players(), games()));
  }

  public useEnrichedGames() {
    const players = this.players.get();
    const games = this.games.get();
    return createMemo(() => enrichGames(games(), players()));
  }

  public getGameHistory(game: number) {
    return createResource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { game: { history: game } } }, message =>
        validateMessage(id, 'history', message)?.history.map(historyFromTuple),
      );
    })[0];
  }

  public renamePlayer(name: string) {
    return this.request({ player: { rename: name } });
  }

  public editGame(game: Game) {
    return this.request({ game: { update: game } });
  }

  public invitePlayer(name: string, email: string) {
    return this.request({ invite: { player: { name, email } } });
  }

  public cancelInvitattion(cancel: number) {
    return this.request({ invite: { cancel } });
  }

  public invitationRsvp(rsvp: boolean) {
    return this.request({ invite: rsvp ? 'accept' : 'reject' });
  }

  public registerGame(
    player: number,
    opponent: number,
    score: number,
    opponentScore: number,
    challenge: boolean,
  ) {
    return this.request({
      game: {
        register: {
          player,
          opponent,
          score,
          opponentScore,
          challenge,
          millis: new Date().getTime(),
        },
      },
    });
  }

  public subscribe(subscriber: Subscriber): Subscriber {
    this.subscribers.push(subscriber);
    return subscriber;
  }

  public unsubscribe(subscriber: Subscriber): Subscriber {
    const idx = this.subscribers.indexOf(subscriber);
    if (idx >= 0) {
      this.subscribers.splice(idx, 1);
    }
    return subscriber;
  }

  private broadcast(message: string, error: boolean) {
    for (const subscriber of this.subscribers) {
      subscriber(message, error);
    }
  }

  private async request<T extends Request['do']>(request: T) {
    const id = newRequestId();
    return this.socket
      .request({ id, do: request }, message => validateDone(id, message))
      .catch((error: unknown) => {
        if (error instanceof ResponseError) {
          this.broadcast(error.message, true);
          return false;
        } else {
          throw error;
        }
      });
  }

  private refresh() {
    this.self.reload();
    this.players.reload();
    this.games.reload();
    this.invites.reload();
  }

  private handlePushPlayer(message: PushPlayer) {
    if ('renamed' in message) {
      const rename = message.renamed;
      this.players.set(players =>
        players.map(p => (p.id === rename.player ? { ...p, name: rename.new } : p)),
      );
      this.broadcast(`Player ${rename.old} changed their name to ${rename.new}`, false);
    } else if ('invited' in message) {
      const invite = message.invited;
      this.invites.set(invites => upsert(invites, invite));
      this.broadcast(`Player ${invite.name} was invited`, false);
    } else if ('uninvited' in message) {
      const uninvite = message.uninvited;
      this.invites.set(invites => invites.filter(i => i.id !== uninvite.id));
      this.broadcast(`Invitation for ${uninvite.name} was lifted`, false);
    } else if ('joined' in message) {
      const join = message.joined;
      this.invites.set(invites => invites.filter(i => i.email !== join.email));
      this.players.set(players => upsert(players, join));
      this.broadcast(`Player ${join.name} joined the fun`, false);
    }
  }

  private handlePushGame(message: PushGame) {
    if ('registered' in message) {
      const game = message.registered.game;

      this.games.set(games => upsert(games, game));
      message.registered.updates.map(gameFromTuple).forEach(game => {
        this.games.set(games => upsert(games, game));
      });

      const players = this.players.raw()?.latest;
      if (players === undefined) {
        return;
      }

      const playerOne = players.find(p => p.id === game.playerOne);
      if (playerOne === undefined) {
        return;
      }

      const playerTwo = players.find(p => p.id === game.playerTwo);
      if (playerTwo === undefined) {
        return;
      }

      if (game.scoreOne > game.scoreTwo) {
        this.broadcast(
          `${playerOne.name} beat ${playerTwo.name} ${game.scoreOne} to ${game.scoreTwo}`,
          false,
        );
      } else {
        this.broadcast(
          `${playerTwo.name} beat ${playerOne.name} ${game.scoreTwo} to ${game.scoreOne}`,
          false,
        );
      }
    } else if ('updated' in message) {
      this.games.set(games => upsert(games, message.updated.game));
      message.updated.updates.map(gameFromTuple).forEach(game => {
        this.games.set(games => upsert(games, game));
      });
    }
  }
}

class Resource<T> {
  private readonly fetcher: () => Promise<T>;
  private readonly mapper?: (data: T) => T;

  private data?: SolidResource<T>;
  private setter?: ResourceActions<T | undefined>;

  constructor(fetcher: () => Promise<T>, mapper?: (data: T) => T) {
    this.fetcher = fetcher;
    this.mapper = mapper;
  }

  public raw(): SolidResource<T> | undefined {
    return this.data;
  }

  public get(): SolidResource<T> {
    if (this.data !== undefined) {
      return this.data;
    }

    const mapper = this.mapper;
    const [data, setter] = createResource(
      mapper === undefined ? this.fetcher : () => this.fetcher().then(d => mapper(d)),
      {
        storage: d =>
          createSignal(d, {
            equals: false,
          }),
      },
    );
    this.data = data;
    this.setter = setter;

    return this.data;
  }

  public set(setter: (current: T) => T) {
    if (this.setter !== undefined) {
      this.setter.mutate(previous => {
        if (previous !== undefined) {
          if (this.mapper === undefined) {
            return setter(previous);
          } else {
            return this.mapper(setter(previous));
          }
        }
        return previous;
      });
    }
  }

  public reload() {
    if (this.setter !== undefined) {
      void this.setter.refetch();
    }
  }
}

const upsert = <T extends Ided>(data: T[], datum: T) => {
  console.debug('Upserting', datum);
  const idx = data.findIndex(d => d.id === datum.id);
  if (idx < 0) {
    console.debug('Insert');
    data.push(datum);
  } else {
    console.debug('Update');
    data[idx] = datum;
  }
  return data;
};

type Subscriber = (message: string, error: boolean) => void;

const enrichPlayers = (players: Player[] = [], games: Game[] = []): EnrichedPlayer[] => {
  const enrichedPlayers = new Map(
    players.map(p => [
      p.id,
      {
        ...p,
        rating: 0,
        games: 0,
        wins: 0,
        losses: 0,
        challengesWon: 0,
        challengesLost: 0,
        pointsWon: 0,
        pointsLost: 0,
      },
    ]),
  );

  for (const game of games.filter(g => !g.deleted)) {
    const playerOne = enrichedPlayers.get(game.playerOne);
    if (playerOne !== undefined) {
      playerOne.rating = game.ratingOne + game.ratingDelta;
      playerOne.games += 1;
      playerOne.pointsWon += game.scoreOne;
      playerOne.pointsLost += game.scoreTwo;
      if (game.scoreOne > game.scoreTwo) {
        playerOne.wins += 1;
        if (game.challenge) {
          playerOne.challengesWon += 1;
        }
      } else {
        playerOne.losses += 1;
        if (game.challenge) {
          playerOne.challengesLost += 1;
        }
      }
    }

    const playerTwo = enrichedPlayers.get(game.playerTwo);
    if (playerTwo !== undefined) {
      playerTwo.rating = game.ratingTwo - game.ratingDelta;
      playerTwo.games += 1;
      playerTwo.pointsLost += game.scoreOne;
      playerTwo.pointsWon += game.scoreTwo;
      if (game.scoreTwo > game.scoreOne) {
        playerTwo.wins += 1;
        if (game.challenge) {
          playerTwo.challengesWon += 1;
        }
      } else {
        playerTwo.losses += 1;
        if (game.challenge) {
          playerTwo.challengesLost += 1;
        }
      }
    }
  }

  return Array.from(enrichedPlayers.values())
    .sort((a, b) => {
      const rating = b.rating - a.rating;
      if (rating !== 0) {
        return rating;
      }

      return b.createdMs - a.createdMs;
    })
    .map((p, i) => {
      return {
        ...p,
        position: i + 1,
      };
    });
};

const enrichGames = (games: Game[] = [], players: Player[] = []): EnrichedGame[] => {
  const playerRatings = new Map(players.map(p => [p.id, { name: p.name, balance: 0 }]));

  return games
    .map(g => {
      const playerOne = playerRatings.get(g.playerOne);
      const playerTwo = playerRatings.get(g.playerTwo);
      const balanceOne = (playerOne?.balance ?? 0) + g.scoreOne - g.scoreTwo;
      const balanceTwo = (playerTwo?.balance ?? 0) + g.scoreTwo - g.scoreOne;

      if (playerOne !== undefined) {
        playerOne.balance = balanceOne;
      }

      if (playerTwo !== undefined) {
        playerTwo.balance = balanceTwo;
      }

      return {
        ...g,
        ratingOne: g.ratingOne + g.ratingDelta,
        ratingTwo: g.ratingTwo - g.ratingDelta,
        balanceOne,
        balanceTwo,
        playerOneName: playerOne?.name,
        playerTwoName: playerTwo?.name,
      };
    })
    .reverse();
};
