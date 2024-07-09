import { ParentProps, Show } from 'solid-js';
import { icon } from '..';
import { type Getter } from '../../util';
import { type Invite, type Player } from '../../types';

import './prompt.css';

export const Prompt = (
  props: ParentProps & {
    visible: () => boolean;
    ok: () => void;
    cancel: () => void;
    disabled?: () => boolean;
  },
) => (
  <Show when={props.visible()}>
    <div class='pages-prompt'>
      <div class='pages-prompt-content'>
        <div class='pages-prompt-form'>{props.children}</div>
        <div class='pages-prompt-buttons'>
          <div
            classList={{
              'pages-prompt-button': true,
              'ok': true,
              'disabled': props.disabled?.() === true,
            }}
            onClick={() => {
              if (props.disabled?.() !== true) {
                props.ok();
              }
            }}
          >
            <icon.Ok />
          </div>
          <div class='pages-prompt-button cancel' onClick={props.cancel}>
            <icon.Cancel />
          </div>
        </div>
      </div>
    </div>
  </Show>
);

export type Props = {
  visible: () => boolean;
  hide: () => void;
};

export const checkAlreadyExists = <T extends 'name' | 'email'>(
  value: string,
  field: T,
  players: Getter<Player[]>,
  invites: Getter<Invite[]>,
) => {
  const trimmed = value.trim();
  if (trimmed === '') {
    return true;
  }

  let index = players()?.findIndex(p => p[field] === trimmed);
  if (index !== undefined && index >= 0) {
    return true;
  }

  index = invites()?.findIndex(p => p[field] === trimmed);
  if (index !== undefined && index >= 0) {
    return true;
  }

  return false;
};
