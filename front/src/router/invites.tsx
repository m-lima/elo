import { For, Suspense } from 'solid-js';
import { A } from '@solidjs/router';

import { Loading } from '../page';
import { icon } from '../components';
import { type Invite, type Player as PlayerType } from '../types';
import { usePlayers, useInvites } from '../store';

import './invites.css';
import { monthToString } from '../util';

type User = {
  readonly id?: number;
  readonly name: string;
  readonly email: string;
  readonly inviter?: number;
  readonly children: User[];
  readonly created: Date;
};

export const Invites = () => {
  const players = usePlayers();
  const invites = useInvites();

  return <Suspense fallback={<Loading />}>{wrapRender(players(), invites())}</Suspense>;
};

const wrapRender = (players: PlayerType[] = [], invites: Invite[] = []) => {
  const roots = players
    .filter(p => p.inviter === undefined)
    .map(p => buildHierarchy(p, players, invites));

  return (
    <>
      <button>Invite</button>
      <div class='router-invites'>
        <For each={roots}>{u => <Player root user={u} />}</For>
      </div>
    </>
  );
};

const buildHierarchy = (player: PlayerType, players: PlayerType[], invites: Invite[]): User => {
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
          created: new Date(i.createdMs),
        };
      }),
  );

  children.sort((a, b) => a.created.getTime() - b.created.getTime());

  return {
    id: player.id,
    name: player.name,
    email: player.email,
    inviter: player.inviter,
    children,
    created: new Date(player.createdMs),
  };
};

const Player = (props: { root?: boolean; user: User }) => (
  <div class='router-invites-player'>
    {props.root || <div class='router-invites-player-line' />}
    <div class='router-invites-player-details'>
      {props.user.id !== undefined ? (
        <>
          <icon.User />
          <A href={`/player/${props.user.id}`}>{props.user.name}</A>
        </>
      ) : (
        <>
          <icon.UserOutline />
          <span>{props.user.name}</span>
        </>
      )}
      {printDate(props.user.created)}
    </div>
    <div class='router-invites'>
      <For each={props.user.children}>{u => <Player user={u} />}</For>
    </div>
  </div>
);

const printDate = (date: Date) =>
  `${date.getDate()}/${String(monthToString(date.getMonth())).padStart(2, '0')}/${date.getFullYear() % 1000}`;
