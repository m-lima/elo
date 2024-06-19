import { For, Suspense } from 'solid-js';

import { Loading } from '../components';
import { usePlayers, useInvites } from '../store';
import { type Invite, type Player } from '../types';

type User = {
  id?: number;
  name: string;
  email: string;
  inviter?: number;
  children: User[];
  createdMs: number;
};

const buildHierarchy = (player: Player, players: Player[], invites: Invite[]): User => {
  const children = players
    .filter(p => p.inviter === player.id)
    .map(p => buildHierarchy(p, players, invites));

  children.push(
    ...invites
      .filter(i => i.inviter === player.id)
      .map(i => {
        return {
          name: i.name,
          email: i.email,
          inviter: i.inviter,
          children: [],
          createdMs: i.createdMs,
        };
      }),
  );

  children.sort((a, b) => a.createdMs - b.createdMs);

  return {
    id: player.id,
    name: player.name,
    email: player.email,
    inviter: player.inviter,
    children,
    createdMs: player.createdMs,
  };
};

const depthToIndent = (depth: number): string => {
  return Array(depth).fill('-').join('');
};

const userRow = (user: User, depth: number) => {
  return (
    <>
      <tr>
        <td>
          {depthToIndent(depth)}
          {user.name}
          {user.id !== undefined ? '' : ' *'}
        </td>
      </tr>
      <For each={user.children}>{u => userRow(u, depth + 1)}</For>
    </>
  );
};

const wrapRender = (players: Player[] = [], invites: Invite[] = []) => {
  const roots = players
    .filter(p => p.inviter === undefined)
    .map(p => buildHierarchy(p, players, invites));

  return (
    <table>
      <tbody>
        <For each={roots}>{u => userRow(u, 0)}</For>
      </tbody>
    </table>
  );
};

export const Invites = () => {
  const players = usePlayers();
  const invites = useInvites();

  return <Suspense fallback={<Loading />}>{wrapRender(players(), invites())}</Suspense>;
};
