import { ParentProps } from 'solid-js';

import { icon } from '.';

import './action.css';

export const Actions = (props: ParentProps) => (
  <div class='components-action'>{props.children}</div>
);

export const Game = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Swords />
    <span class='components-action-text'>New game</span>
  </div>
);

export const Invite = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.User />
    <span class='components-action-text'>Invite</span>
  </div>
);

export const Edit = (props: { text: string; action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Edit />
    <span class='components-action-text'>{props.text}</span>
  </div>
);

export const Delete = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Trash />
    <span class='components-action-text'>Delete</span>
  </div>
);

export const Restore = (props: { action: () => void }) => (
  <div class='components-action-content' onClick={props.action}>
    <icon.Restore />
    <span class='components-action-text'>Restore</span>
  </div>
);
