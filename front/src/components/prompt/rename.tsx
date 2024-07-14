import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';
import { type Getter, type Invite, type Player } from '../../types';

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

  const commit = () => {
    if (invalid()) {
      return;
    }

    props.store.renamePlayer(name().trim()).then(r => {
      if (r) {
        props.hide();
        setName('');
      }
    });
  };

  return (
    <Prompt
      visible={props.visible}
      ok={commit}
      cancel={props.hide}
      disabled={() => invalid() !== CheckResult.Ok}
    >
      <div class='components-prompt-rename'>
        <b>Name</b>
        <input
          class={invalid() === CheckResult.Conflict ? 'invalid' : undefined}
          type='text'
          autofocus
          placeholder={props.name}
          value={name()}
          onInput={e => setName(e.currentTarget.value)}
          onChange={e => setName(e.currentTarget.value.trim())}
          onKeyDown={e => {
            if (e.key === 'Enter') {
              commit();
            } else if (e.key === 'Escape') {
              props.hide();
            }
          }}
        />
      </div>
    </Prompt>
  );
};
