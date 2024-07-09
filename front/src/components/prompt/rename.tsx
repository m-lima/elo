import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';
import { type Invite, type Player } from '../../types';
import { type Getter } from '../../util';

import { Prompt, checkAlreadyExists, type Props } from './prompt';

import './rename.css';

export const Rename = (
  props: Props & {
    store: Store;
    name: string;
    players: Getter<Player[]>;
    invites: Getter<Invite[]>;
  },
) => {
  const [name, setName] = createSignal(props.name);

  const invalid = createMemo(() =>
    checkAlreadyExists(name(), 'name', props.players, props.invites),
  );

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
      disabled={invalid}
    >
      <div class='components-prompt-rename'>
        <b>Name</b>
        <input
          class={invalid() ? 'invalid' : undefined}
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
