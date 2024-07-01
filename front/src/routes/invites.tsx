import { For, Suspense } from 'solid-js';
import { A } from '@solidjs/router';

import { Loading, Main } from '../pages';
import { Action, Actions, icon } from '../components';
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
      <Actions>
        <Action
          icon={<icon.Add />}
          text='New invite'
          action={() => {
            console.debug('Clicked');
          }}
        />
      </Actions>
      <Main>
        <div class='routes-invites' id='main'>
          <For each={roots}>{u => <Player root user={u} />}</For>
        </div>
      </Main>
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
  <div class='routes-invites-player'>
    {props.root || <div class='routes-invites-player-line' />}
    <div class='routes-invites-player-details'>
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
    <div class='routes-invites'>
      <For each={props.user.children}>{u => <Player user={u} />}</For>
    </div>
  </div>
);

const printDate = (date: Date) =>
  `${date.getDate()}/${String(monthToString(date.getMonth())).padStart(2, '0')}/${date.getFullYear() % 1000}`;
