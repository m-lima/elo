import { Component } from 'solid-js';
import { Router, Navigate, Route } from '@solidjs/router';

import { Games } from './games';
import { Game } from './game';
import { Invites } from './invites';
import { Leaderboard } from './leaderboard';
import { Player } from './player';

export const Routes = (props: { root: Component }) => (
  <Router root={props.root}>
    <Route path='/games' component={Games} />
    <Route path='/game/:id' component={Game} />
    <Route path='/invites' component={Invites} />
    <Route path='/player/:id?' component={Player} />
    <Route path='/' component={Leaderboard} />
    <Route path='*' component={() => <Navigate href='/' />} />
  </Router>
);
