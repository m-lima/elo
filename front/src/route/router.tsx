import { HashRouter, Route } from '@solidjs/router';

import { Main } from './main';

import './router.css';

export const Root = () =>
  <div class='router'>
    <HashRouter root={Main}>
      <Route path='/' component={Main} />
    </HashRouter>;
  </div>;
