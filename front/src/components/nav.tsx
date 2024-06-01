import { A } from '@solidjs/router';
import { Suspense, createMemo } from 'solid-js';

import { useSelf } from '../store';

import './nav.css';

export const Nav = () => {
  const self = useSelf();

  const medal = createMemo(() => {
    switch (self()?.position) {
      case 1:
        return '🥇 ';
      case 2:
        return '🥈 ';
      case 3:
        return '🥉 ';
      default: return '';
    }
  });

  return (
    <nav class='components nav root'>
      <ul>
        <li>
          <A class='components nav name' href='/'>
            <strong>PongElo</strong>
          </A>
        </li>
      </ul>
      <ul>
        <Suspense>
          <li><A href='/user' class='secondary'>{medal()}#{self()?.position}</A></li>
          <li><A href='/user' class='secondary'>{self()?.score}</A></li>
        </Suspense>
      </ul>
    </nav>
  );
  // return <div class='components nav'>
  //   <Suspense fallback={<h1>Loading</h1>} >
  //     <div>
  //       <h1>User</h1>
  //       <h3>Id</h3>
  //       {self()?.id}
  //       <h3>Name</h3>
  //       {self()?.name}
  //       <h3>Email</h3>
  //       {self()?.email}
  //     </div>
  //     <A href='/'>Home</A>
  //     <A href='/bla'>Bla</A>
  //   </Suspense>
  // </div>;
};
