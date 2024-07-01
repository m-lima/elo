import { JSXElement, createSignal } from 'solid-js';

import * as util from '../util';
import { useStore } from '../store';

import './welcome.css';

const Prompt = (props: {
  position?: 'left' | 'right';
  header: string;
  text: JSXElement;
  ok: {
    text: string;
    action: () => void;
  };
  cancel: {
    text: string;
    action: () => void;
  };
}) => (
  <div class={`pages-welcome-prompt ${props.position}`}>
    <h1>{props.header}</h1>
    <h3>{props.text}</h3>
    <div class='pages-welcome-prompt-buttons'>
      <button onClick={props.ok.action}>{props.ok.text}</button>
      <button class='secondary' onClick={props.cancel.action}>
        {props.cancel.text}
      </button>
    </div>
  </div>
);

export const Welcome = () => {
  const [willAccept, setWillAccept] = createSignal<boolean | undefined>();
  const store = useStore();

  return (
    <div class='pages-welcome' id='main' style={{ overflow: 'hidden' }}>
      <Prompt
        position={willAccept() !== undefined ? 'left' : undefined}
        header="You've been invited!!"
        text=<>
          In order to access <b>{util.name}</b>, you must accept the invitation
        </>
        ok={{
          text: 'Accept',
          action: () => {
            if (willAccept() === undefined) {
              setWillAccept(true);
            }
          },
        }}
        cancel={{
          text: 'Reject',
          action: () => {
            if (willAccept() === undefined) {
              setWillAccept(false);
            }
          },
        }}
      />
      <Prompt
        position={willAccept() !== true ? 'right' : undefined}
        header='Great news!!'
        text='Just to make sure, can you accept again?'
        ok={{
          text: 'Accept',
          action: () => {
            if (willAccept() === true) {
              void store.invitationRsvp(true).finally(() => {
                window.location.reload();
              });
            }
          },
        }}
        cancel={{
          text: 'Oops',
          action: () => {
            if (willAccept() === true) {
              setWillAccept();
            }
          },
        }}
      />
      <Prompt
        position={willAccept() !== false ? 'right' : undefined}
        header='Too bad..'
        text='Just to make sure, can you reject again?'
        ok={{
          text: 'Reject',
          action: () => {
            if (willAccept() === false) {
              void store.invitationRsvp(false).finally(() => {
                window.location.reload();
              });
            }
          },
        }}
        cancel={{
          text: 'Oops',
          action: () => {
            if (willAccept() === false) {
              setWillAccept();
            }
          },
        }}
      />
    </div>
  );
};
