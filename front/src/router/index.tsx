import { Component } from 'solid-js';
import { Router as SolidRouter, Navigate, Route } from '@solidjs/router';

import { Home } from './home';

export { Loading } from './loading';

export const Router = (props: { root: Component }) => (
  <SolidRouter base='/' root={props.root}>
    <Route path='/bla' component={() => <h1>Yoooo</h1>} />
    <Route path='/' component={Home} />
    <Route path='*' component={() => <Navigate href='/' />} />
  </SolidRouter>
);
