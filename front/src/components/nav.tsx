import { Suspense } from 'solid-js';

import { useSelf } from '../store';

import { Spinner } from '.';

import './nav.css';

export const Nav = () => {
  const self = useSelf();

  // const medal = createMemo(() => {
  //   switch (self()?.position) {
  //     case 1:
  //       return '🥇 ';
  //     case 2:
  //       return '🥈 ';
  //     case 3:
  //       return '🥉 ';
  //     default: return '';
  //   }
  // });

  return (
    <nav class='components nav root'>
      <strong>PongElo</strong>
      <Suspense fallback={<Spinner size='1em' />}>
        <span>
          #{self()?.position} ({self()?.score} pts)
        </span>
      </Suspense>
    </nav>
  );
};
