import { A } from '@solidjs/router';
import { JSXElement, Show, createSignal } from 'solid-js';

import { icon } from '.';

import './side.css';

const Item = (props: { icon: JSXElement; text: string; visible: boolean }) => (
  <>
    {props.icon}
    <span class='components-side-text' id={props.visible ? 'visible' : ''}>
      {props.text}
    </span>
  </>
);

// TODO: Cookie for expanded setting
// TODO: Setting for notifications
export const Side = () => {
  const [expanded, setExpanded] = createSignal(true);

  return (
    <aside class='components-side' id='side'>
      <A href='/' end>
        <Item icon=<icon.Trophy /> text='Leaderboard' visible={expanded()} />
      </A>
      <A href='/player'>
        <Item icon=<icon.User /> text='Player' visible={expanded()} />
      </A>
      <A href='/games'>
        <Item icon=<icon.PingPong /> text='Games' visible={expanded()} />
      </A>
      <A href='/invites'>
        <Item icon=<icon.Hierarchy /> text='Invites' visible={expanded()} />
      </A>
      <span onClick={() => setExpanded(e => !e)}>
        <Show
          when={expanded()}
          fallback=<Item icon=<icon.DoubleRight /> text='Collapse' visible={false} />
        >
          <Item icon=<icon.DoubleLeft /> text='Collapse' visible={true} />
        </Show>
      </span>
    </aside>
  );
};
