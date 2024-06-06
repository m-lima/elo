import { Show, Suspense, createMemo } from 'solid-js';
import { useParams } from '@solidjs/router';

import { usePlayers, useSelf } from '../store';
import { Player as PlayerType } from '../types';

import { Loading } from '.';

const View = (props: { id?: number; self: PlayerType; players: PlayerType[] }) => {
  const actualId = props.id !== undefined ? props.id : props.self.id;

  return (
    <>
      <p>Self: {props.self.id}</p>
      <p>Param: {actualId}</p>
    </>
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
        <View id={id()} self={self()!} players={players()!} />
      </Show>
    </Suspense>
  );
};
