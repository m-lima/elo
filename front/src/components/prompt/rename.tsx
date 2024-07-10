import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';
import { type Invite, type Player } from '../../types';
import { type Getter } from '../../util';

import { CheckResult, Prompt, type Props, checkString } from './prompt';

import './rename.css';

export const Rename = (
  props: Props & {
    store: Store;
    name: string;
    players: Getter<Player[]>;
    invites: Getter<Invite[]>;
  },
) => {
  const [name, setName] = createSignal('');

  const invalid = createMemo(() => checkString(name(), 'name', props.players, props.invites));

  return (
    <Prompt
      visible={props.visible}
      ok={() => {
        props.store.renamePlayer(name().trim()).then(r => {
          if (r) {
            props.hide();
          }
        });
      }}
      cancel={props.hide}
      disabled={() => invalid() !== CheckResult.Ok}
    >
      <div class='components-prompt-rename'>
        <b>Name</b>
        <input
          class={invalid() === CheckResult.Conflict ? 'invalid' : undefined}
          type='text'
          placeholder={props.name}
          value={name()}
          onInput={e => setName(e.currentTarget.value)}
          onChange={e => setName(e.currentTarget.value.trim())}
        />
      </div>
    </Prompt>
  );
};
