import { createSignal, Match, Switch } from 'solid-js';

import * as util from '../util';
import { useStore } from '../store';

import './invite.css';

const Initial = (props: { decider: (decision: boolean) => void }) => (
  <div class='components-invite'>
    <h1>You've been invited!</h1>
    <h3>
      In order to access <b>{util.name}</b>, you must accept the invitation
    </h3>
    <div class='components-invite-buttons'>
      <button
        onClick={() => {
          props.decider(true);
        }}
      >
        Accept
      </button>
      <button
        class='secondary'
        onClick={() => {
          props.decider(false);
        }}
      >
        Reject
      </button>
    </div>
  </div>
);

const WillAccept = (props: { decider: (decision: boolean) => void }) => (
  <div class='components-invite'>
    <h1>Great news!</h1>
    <h3>Just to make sure, can you accept again?</h3>
    <div class='components-invite-buttons'>
      <button
        onClick={() => {
          props.decider(true);
        }}
      >
        Accept
      </button>
      <button
        class='secondary'
        onClick={() => {
          props.decider(false);
        }}
      >
        {' '}
        Oops
      </button>
    </div>
  </div>
);

const WillReject = (props: { decider: (decision: boolean) => void }) => (
  <div class='components-invite'>
    <h1>Too bad..</h1>
    <h3>Just to make sure, can you reject again?</h3>
    <div class='components-invite-buttons'>
      <button
        onClick={() => {
          props.decider(true);
        }}
      >
        Reject
      </button>
      <button
        class='secondary'
        onClick={() => {
          props.decider(false);
        }}
      >
        Oops
      </button>
    </div>
  </div>
);

export const Invite = () => {
  const [willAccept, setWillAccept] = createSignal<boolean | undefined>();
  const store = useStore();

  return (
    <div>
      <div class={`components-invite right ${willAccept() === undefined && 'visible'}`}>
        <Initial
          decider={decision => {
            if (willAccept() === undefined) {
              setWillAccept(decision);
            }
          }}
        />
      </div>
      <div class={`components-invite left ${willAccept() === true && 'visible'}`}>
        <WillAccept
          decider={decision => {
            if (willAccept() === true) {
              if (decision) {
                void store.invitationRsvp(true).finally(() => {
                  window.location.reload();
                });
              } else {
                setWillAccept();
              }
            }
          }}
        />
      </div>
      <div class={`components-invite left ${willAccept() === false && 'visible'}`}>
        <WillReject
          decider={decision => {
            if (willAccept() === false) {
              if (decision) {
                void store.invitationRsvp(false).finally(() => {
                  window.location.reload();
                });
              } else {
                setWillAccept();
              }
            }
          }}
        />
      </div>
    </div>
  );
};
