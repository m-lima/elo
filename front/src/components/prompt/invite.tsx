import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';

import { Prompt, type Props } from './prompt';

import './invite.css';

export const Invite = (props: Props & { store: Store }) => {
  const [name, setName] = createSignal('');
  const [email, setEmail] = createSignal('');

  const invalidName = createMemo(() => {
    if (name().trim() === '') {
      return true;
    }

    return false;
  });

  const invalidEmail = createMemo(() => {
    const parts = email().split('@');
    if (parts.length !== 2 || parts[0] === '' || parts[1] === '') {
      return true;
    }

    return false;
  });

  return (
    <Prompt
      ok={() => {
        props.store.invitePlayer(name(), email()).then(props.hide);
      }}
      cancel={props.hide}
      disabled={() => invalidName() || invalidEmail()}
    >
      <div class='components-prompt-invite'>
        <b>Name</b>
        <input
          class={invalidName() ? 'invalid' : undefined}
          type='text'
          placeholder='Name'
          value={name()}
          onInput={e => setName(e.currentTarget.value)}
          onChange={e => setName(e.currentTarget.value.trim())}
        />
        <b>Email</b>
        <input
          class={invalidEmail() ? 'invalid' : undefined}
          type='text'
          placeholder='Email'
          value={email()}
          onInput={e => setEmail(e.currentTarget.value)}
          onChange={e => setEmail(e.currentTarget.value.trim())}
        />
      </div>
    </Prompt>
  );
};
