import { ParentProps, Show } from 'solid-js';
import { icon } from '..';
import { type Getter, type Invite, type Player } from '../../types';
import { Loading } from '../../pages';

import './prompt.css';

export const Prompt = (
  props: ParentProps & {
    visible: () => boolean;
    ok: () => void;
    cancel: () => void;
    disabled: () => boolean;
    busy: () => boolean | undefined;
  },
) => (
  <Show when={props.visible()}>
    <div class='components-prompt'>
      <Show when={props.busy() !== true} fallback=<Loading />>
        <div class='grid'>
          <div class='content'>
            <Show when={props.children !== undefined}>
              <div class='form'>{props.children}</div>
            </Show>
            <div class='buttons'>
              <div
                classList={{
                  button: true,
                  ok: true,
                  disabled: props.disabled(),
                }}
                onClick={() => {
                  if (!props.disabled()) {
                    props.ok();
                  }
                }}
              >
                <icon.Ok />
              </div>
              <div class='button cancel' onClick={props.cancel}>
                <icon.Cancel />
              </div>
            </div>
          </div>
        </div>
      </Show>
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
