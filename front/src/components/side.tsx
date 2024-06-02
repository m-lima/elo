import { A } from '@solidjs/router';
import { JSXElement, Match, Switch, createSignal } from 'solid-js';

import { useSelf } from '../store';
import { icon } from '.';

import './side.css';

const Item = (props: { path: string, icon: JSXElement, text: string, visible: boolean }) =>
  <A href={props.path}>
    {props.icon} <span class='components_side_text' id={props.visible ? 'visible' : ''}>{props.text}</span>
  </A>;

export const Side = () => {
  const self = useSelf();
  const [expanded, setExpanded] = createSignal(false);

  return (
    <aside class='components_side'>
      <Item path='/' icon={<icon.Trophy />} text='Leaderboard' visible={expanded()} />
      <Item path={self() ? `/user/${self()!.id}` : window.location.toString()} icon={<icon.User />} text='User' visible={expanded()} />
      <span onClick={() => setExpanded(e => !e)}>
        <Switch>
          <Match when={expanded()}>
            <icon.DoubleLeft />
          </Match>
          <Match when={!expanded()}>
            <icon.DoubleRight />
          </Match>
        </Switch> <span class='components_side_text' id={expanded() ? 'visible' : ''}>Collapse</span>
      </span>
    </aside >
  );
}
