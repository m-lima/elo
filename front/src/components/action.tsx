import { JSXElement, ParentProps } from 'solid-js';

import './action.css';

export const Actions = (props: ParentProps) => (
  <div class='components-action' id='actions'>
    {props.children}
  </div>
);

export const Action = (props: { icon: JSXElement; text: string; action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    {props.icon}
    <span class='components-action-text'>{props.text}</span>
  </div>
);
