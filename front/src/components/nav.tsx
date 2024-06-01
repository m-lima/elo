import { A } from '@solidjs/router';
import { Resource, Suspense, createMemo } from 'solid-js';

import logo from '/logo.svg';

import { useSelf } from '../store';
import { User } from '../types';

import './nav.css';

export const Nav = () => {
  const self = useSelf();

  const toSummary = (resource: Resource<User>): Resource<string> => {
    resource.call
  }

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
    <nav>
      <ul>
        <li>
          <A class='components nav name' href='/'>
            <img class='components nav logo' src={logo} />
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
