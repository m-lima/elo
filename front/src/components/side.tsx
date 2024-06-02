import { A } from '@solidjs/router';
import { JSXElement, Match, Switch, createSignal } from 'solid-js';

import { useSelf } from '../store';
import { icon } from '.';

import './side.css';

const Item = (props: { path: string, icon: JSXElement, text?: false | string }) =>
  <A href={props.path}>
    {props.icon} <span class='components_side_text' id={props.text ? 'visible' : undefined}>{props.text}</span>
  </A>;

export const Side = () => {
  const self = useSelf();
  const [expanded, setExpanded] = createSignal(false);

  return (
    <aside class='components_side'>
      <Item path='/' icon={<icon.Trophy />} text={expanded() && 'Leaderboard'} />
      <Item path={self() ? `/user/${self()!.id}` : `${window.location}`} icon={<icon.User />} text={expanded() && 'User'} />
      <Switch>
        <Match when={expanded()}>
          <span onClick={() => setExpanded(false)}>
            <icon.DoubleLeft /> <span class='components_side_text' id='visible'>Collapse</span>
          </span>
        </Match>
        <Match when={!expanded()}>
          <span onClick={() => setExpanded(true)}>
            <icon.DoubleRight />
          </span>
        </Match>
      </Switch>
    </aside >
  );
}
