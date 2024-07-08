import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';

import { Prompt, type Props } from './prompt';

import './invite.css';

export const Invite = (props: Props & { store: Store }) => {
  const [name, setName] = createSignal('');
  const [email, setEmail] = createSignal('');

  const invalidName = createMemo(() => {
    const maybeName = name();
    if (maybeName === undefined) {
      return true;
    }

    if (maybeName.trim() === '') {
      return true;
    }

    return false;
  });

  const invalidEmail = createMemo(() => {
    const maybeEmail = email();
    if (maybeEmail === undefined) {
      return true;
    }

    if (maybeEmail.trim() === '') {
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
