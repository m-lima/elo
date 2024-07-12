import { createMemo, createSignal } from 'solid-js';

import { Store } from '../../store';
import { type Getter, type Invite as InviteType, type Player } from '../../types';

import { CheckResult, Prompt, checkString, type Props } from './prompt';

import './invite.css';

export const Invite = (
  props: Props & { store: Store; players: Getter<Player[]>; invites: Getter<InviteType[]> },
) => {
  const [name, setName] = createSignal('');
  const [email, setEmail] = createSignal('');

  const invalidName = createMemo(() => checkString(name(), 'name', props.players, props.invites));

  const invalidEmail = createMemo(() => {
    const clean = cleanEmail(email());
    const parts = clean.split('@');
    if (parts.length !== 2 || parts[0] === '' || parts[1] === '') {
      return CheckResult.Empty;
    }

    return checkString(clean, 'email', props.players, props.invites);
  });

  return (
    <Prompt
      visible={props.visible}
      ok={() => {
        props.store.invitePlayer(name().trim(), cleanEmail(email())).then(r => {
          if (r) {
            props.hide();
          }
        });
      }}
      cancel={props.hide}
      disabled={() => invalidName() !== CheckResult.Ok || invalidEmail() !== CheckResult.Ok}
    >
      <div class='components-prompt-invite'>
        <b>Name</b>
        <input
          class={invalidName() === CheckResult.Conflict ? 'invalid' : undefined}
          type='text'
          placeholder='Name'
          value={name()}
          onInput={e => setName(e.currentTarget.value)}
          onChange={e => setName(e.currentTarget.value.trim())}
        />
        <b>Email</b>
        <input
          class={invalidEmail() === CheckResult.Conflict ? 'invalid' : undefined}
          type='text'
          placeholder='Email'
          value={email()}
          onInput={e => setEmail(e.currentTarget.value)}
          onChange={e => {
            setEmail(cleanEmail(e.currentTarget.value));
          }}
        />
      </div>
    </Prompt>
  );
};

const cleanEmail = (email: string) =>
  email
    .split('@')
    .map(p => p.trim())
    .join('@');
