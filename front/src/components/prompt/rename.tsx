import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';

import { Prompt, type Props } from './prompt';

import './rename.css';

export const Rename = (props: Props & { store: Store; name: string }) => {
  const [name, setName] = createSignal(props.name);

  const invalid = createMemo(() => {
    if (name().trim() === '') {
      return true;
    }

    return false;
  });

  return (
    <Prompt
      ok={() => {
        props.store.renamePlayer(name()).then(props.hide);
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
