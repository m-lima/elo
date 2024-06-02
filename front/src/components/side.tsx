import { A } from '@solidjs/router';
import { ComponentProps } from 'solid-js';

import { useSelf } from '../store';
import { icon } from '.';

import './side.css';

export const Side = (props: ComponentProps<'aside'>) => {
  const self = useSelf();

  return (
    <aside class='components_side' {...props}>
      <A href='/'><icon.Trophy /></A>
      <A href={self() ? `/user'${self()!.id}` : window.location}><icon.User /></A>
      <A href='/'>h</A>
    </aside >
  );
}
