import { A } from '@solidjs/router';
import { JSXElement, Show, createSignal } from 'solid-js';

import { icon } from '.';

import './side.css';

const Item = (props: { icon: JSXElement; text: string; visible: boolean }) => (
  <>
    {props.icon}
    <span class='components_side_text' id={props.visible ? 'visible' : ''}>
      {props.text}
    </span>
  </>
);

export const Side = () => {
  const [expanded, setExpanded] = createSignal(false);

  return (
    <aside class='components_side'>
      <A href='/' end>
        <Item icon={<icon.Trophy />} text='Leaderboard' visible={expanded()} />
      </A>
      <A href='/test' end>
        <Item icon={<icon.Mosquito />} text='Test' visible={expanded()} />
      </A>
      <A href='/player/'>
        <Item icon={<icon.User />} text='Player' visible={expanded()} />
      </A>
      <span onClick={() => setExpanded(e => !e)}>
        <Show
          when={expanded()}
          fallback={<Item icon={<icon.DoubleRight />} text='Collapse' visible={false} />}
        >
          <Item icon={<icon.DoubleLeft />} text='Collapse' visible={true} />
        </Show>
      </span>
    </aside>
  );
};
