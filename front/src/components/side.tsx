import { A } from '@solidjs/router';
import { JSXElement, Show, Suspense, createSignal } from 'solid-js';

import { useSelf } from '../store';
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
  const self = useSelf();
  const [expanded, setExpanded] = createSignal(false);
  const userPath = (id?: number) => (id !== undefined ? `/user/${id}` : '');

  return (
    <aside class='components_side'>
      <A href='/'>
        <Item icon={<icon.Trophy />} text='Leaderboard' visible={expanded()} />
      </A>
      <Suspense
        fallback={
          <span class='components_side_ignore'>
            <Item icon={<icon.Spinner />} text='Loading' visible={expanded()} />
          </span>
        }
      >
        <A href={userPath(self()?.id)}>
          <Item icon={<icon.User />} text='User' visible={expanded()} />
        </A>
      </Suspense>
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
