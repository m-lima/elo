import { ParentProps } from 'solid-js';

import './main.css';

export const Main = (props: ParentProps) => (
  <div class='pages-main' id='main'>
    {props.children}
  </div>
);
