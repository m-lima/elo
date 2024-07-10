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
    <div class='components-prompt'>
      <div class='components-prompt-content'>
        <div class='components-prompt-form'>{props.children}</div>
        <div class='components-prompt-buttons'>
          <div
            classList={{
              'components-prompt-button': true,
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
          <div class='components-prompt-button cancel' onClick={props.cancel}>
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

export const checkString = <T extends 'name' | 'email'>(
  value: string,
  field: T,
  players: Getter<Player[]>,
  invites: Getter<Invite[]>,
) => {
  const trimmed = value.trim();
  if (trimmed === '') {
    return CheckResult.Empty;
  }

  let index = players()?.findIndex(p => p[field] === trimmed);
  if (index !== undefined && index >= 0) {
    return CheckResult.Conflict;
  }

  index = invites()?.findIndex(p => p[field] === trimmed);
  if (index !== undefined && index >= 0) {
    return CheckResult.Conflict;
  }

  return CheckResult.Ok;
};

export enum CheckResult {
  Ok,
  Conflict,
  Empty,
}
