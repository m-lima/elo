import { A } from '@solidjs/router';
import { Suspense } from 'solid-js';

import { useSelf } from '../store';

import './nav.css';

export const Nav = () => {
  const self = useSelf();

  return <div class='components nav'>
    <Suspense fallback={<h1>Loading</h1>} >
      <div>
        <h1>User</h1>
        <h3>Id</h3>
        {self()?.id}
        <h3>Name</h3>
        {self()?.name}
        <h3>Email</h3>
        {self()?.email}
      </div>
      <A href='/'>Home</A>
      <A href='/bla'>Bla</A>
    </Suspense>
  </div>;
};
