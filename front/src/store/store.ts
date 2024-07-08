import { createResource, createSignal, ResourceActions, Resource as SolidResource } from 'solid-js';

import { Socket, state } from '../socket';
import {
  type Player,
  type Game,
  type Invite,
  type User,
  playerFromTuple,
  gameFromTuple,
  inviteFromTuple,
} from '../types';
import { type Message, type Request, type Ided } from './message';
import { newRequestId, ResponseError, validateDone, validateMessage } from './request';
import { sortPlayers } from '../util';

export class Store {
  private readonly socket: Socket<Request, Message>;

  private readonly subscribers: Subscriber[];

  private readonly self: Resource<User>;
  private readonly players: Resource<Player[]>;
  private readonly games: Resource<Game[]>;
  private readonly invites: Resource<Invite[]>;

  public static makeSocket(url: string | URL, checkUrl?: string | URL): Socket<Request, Message> {
    return new Socket(url, checkUrl);
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
          if ('renamed' in message.push.player) {
            const rename = message.push.player.renamed;
            this.players.set(players =>
              players.map(p => (p.id === rename.player ? { ...p, name: rename.new } : p)),
            );
            this.broadcast(`Player ${rename.old} changed their name to ${rename.new}`, false);
          } else if ('invited' in message.push.player) {
            const invite = message.push.player.invited;
            this.invites.set(invites => upsert(invites, invite));
            this.broadcast(`Player ${invite.name} was invited`, false);
          } else if ('uninvited' in message.push.player) {
            const uninvite = message.push.player.uninvited;
            this.invites.set(invites => invites.filter(i => i.id !== uninvite.id));
            this.broadcast(`Invitation for ${uninvite.name} was lifted`, false);
          } else if ('joined' in message.push.player) {
            const join = message.push.player.joined;
            this.invites.set(invites => invites.filter(i => i.email !== join.email));
            this.players.set(players => upsert(players, join));
            this.broadcast(`Player ${join.name} joined the fun`, false);
          }
        } else if ('game' in message.push) {
          if ('registered' in message.push.game) {
            const [game, playerOne, playerTwo] = message.push.game.registered;
            this.games.set(games => upsert(games, game));
            this.players.set(players => {
              upsert(players, playerOne);
              upsert(players, playerTwo);
              return players;
            });
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
          }
        }
      }
      return true;
    });

    this.subscribers = [];

    this.self = new Resource(() => {
      const id = newRequestId();
      return this.socket.request({ id, do: { player: 'id' } }, message => {
        const validated = this.wrapValidation(() =>
          validateMessage(id, ['user', 'pending'], message),
        );

        if (validated === undefined) {
          return;
        }

        if (validated.user !== undefined) {
          return { id: validated.user, pending: false };
        } else if (validated.pending !== undefined) {
          return { id: validated.pending, pending: true };
        }
      });
    });

    this.players = new Resource(
      () => {
        const id = newRequestId();
        return this.socket.request({ id, do: { player: 'list' } }, message => {
          const validated = this.wrapValidation(() => validateMessage(id, 'players', message));

          if (validated === undefined) {
            return;
          }

          return validated.players.map(playerFromTuple);
        });
      },
      players => players.sort(sortPlayers),
    );

    this.games = new Resource(
      () => {
        const id = newRequestId();
        return this.socket.request({ id, do: { game: 'list' } }, message => {
          const validated = this.wrapValidation(() => validateMessage(id, 'games', message));

          if (validated === undefined) {
            return;
          }

          return validated.games.map(gameFromTuple);
        });
      },
      games => games.sort((a, b) => b.createdMs - a.createdMs),
    );

    this.invites = new Resource(
      () => {
        const id = newRequestId();
        return this.socket.request({ id, do: { invite: 'list' } }, message => {
          const validated = this.wrapValidation(() => validateMessage(id, 'invites', message));

          if (validated === undefined) {
            return;
          }

          return validated.invites.map(inviteFromTuple);
        });
      },
      invites => invites.sort((a, b) => b.createdMs - a.createdMs),
    );
  }

  public getSelf() {
    return this.self.get();
  }

  public getPlayers() {
    return this.players.get();
  }

  public getGames() {
    return this.games.get();
  }

  public getInvites() {
    return this.invites.get();
  }

  public renamePlayer(name: string) {
    const id = newRequestId();
    return this.socket.request({ id, do: { player: { rename: name } } }, message =>
      this.wrapValidation(() => validateDone(id, message)),
    );
  }

  public invitePlayer(name: string, email: string) {
    const id = newRequestId();
    return this.socket.request({ id, do: { invite: { player: { name, email } } } }, message =>
      this.wrapValidation(() => validateDone(id, message)),
    );
  }

  public cancelInvitattion(cancel: number) {
    const id = newRequestId();
    return this.socket.request({ id, do: { invite: { cancel } } }, message =>
      this.wrapValidation(() => validateDone(id, message)),
    );
  }

  public invitationRsvp(rsvp: boolean) {
    const id = newRequestId();
    return this.socket.request({ id, do: { invite: rsvp ? 'accept' : 'reject' } }, message =>
      this.wrapValidation(() => validateDone(id, message)),
    );
  }

  public registerGame(opponent: number, score: number, opponentScore: number, challenge: boolean) {
    const id = newRequestId();
    return this.socket.request(
      { id, do: { game: { register: { opponent, score, opponentScore, challenge } } } },
      message => this.wrapValidation(() => validateDone(id, message)),
    );
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

  private refresh() {
    this.self.reload();
    this.players.reload();
    this.games.reload();
    this.invites.reload();
  }

  private wrapValidation<T>(validation: () => T) {
    try {
      return validation();
    } catch (e) {
      console.debug('Validation', e);
      if (e instanceof ResponseError) {
        console.debug('Validation expected');
        this.broadcast(e.message, true);
      }
      throw e;
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
      this.setter.refetch();
    }
  }
}

const upsert = <T extends Ided>(data: T[], datum: T) => {
  const idx = data.findIndex(d => d.id === datum.id);
  if (idx < 0) {
    data.push(datum);
  } else {
    data[idx] = datum;
  }
  return data;
};

type Subscriber = (message: string, error: boolean) => void;
