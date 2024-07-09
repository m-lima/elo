import { ParentProps } from 'solid-js';

import { icon } from '.';

import './action.css';

export const Actions = (props: ParentProps) => (
  <div class='components-action'>{props.children}</div>
);

export const Game = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Add />
    <span class='components-action-text'>Game</span>
  </div>
);

export const Invite = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Add />
    <span class='components-action-text'>Invite</span>
  </div>
);

export const Rename = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Edit />
    <span class='components-action-text'>Rename</span>
  </div>
);
