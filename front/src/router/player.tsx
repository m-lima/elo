import { Show, Suspense, createMemo } from 'solid-js';
import { useParams } from '@solidjs/router';

import { usePlayers, useSelf } from '../store';
import { Player as PlayerType } from '../types';
import { Loading } from '../components';

const Loaded = (props: { selfId?: number; self: PlayerType; players: PlayerType[] }) => {
  const actualId = props.selfId !== undefined ? props.selfId : props.self.id;
  const actualPlayer = props.players.find(p => p.id === actualId);

  return (
    <Show when={actualPlayer}>
      <>
        <h1>{actualPlayer?.name}</h1>
        <p>{actualPlayer?.email}</p>
        <p>Self: {props.self.id}</p>
        <p>Param: {actualId}</p>
      </>
    </Show>
  );
};

export const Player = () => {
  const params = useParams<{ id?: string }>();
  const self = useSelf();
  const players = usePlayers();

  const id = createMemo(() => {
    const id: number = Number(params.id);
    return isNaN(id) ? undefined : id;
  });

  return (
    <Suspense fallback={<Loading />}>
      <Show when={self() !== undefined && players() !== undefined}>
        <Loaded selfId={id()} self={self()!} players={players()!} />
      </Show>
    </Suspense>
  );
};
