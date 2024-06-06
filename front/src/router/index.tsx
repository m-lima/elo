import { Component } from 'solid-js';
import { Router as SolidRouter, Navigate, Route } from '@solidjs/router';

import { Home } from './home';
import { Leaderboard } from './leaderboard';
import { Player } from './player';
export { Loading } from './loading';

export const Router = (props: { root: Component }) => (
  <SolidRouter root={props.root}>
    <Route path='/test' component={Home} />
    <Route path='/player/:id?' component={Player} />
    <Route path='/' component={Leaderboard} />
    <Route path='*' component={() => <Navigate href='/' />} />
  </SolidRouter>
);
