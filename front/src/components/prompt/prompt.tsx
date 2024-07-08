import { ParentProps } from 'solid-js';
import { icon } from '..';

import './prompt.css';

export const Prompt = (
  props: ParentProps & { ok: () => void; cancel: () => void; disabled?: () => boolean },
) => (
  <div class='pages-prompt'>
    <div class='pages-prompt-content'>
      <div class='pages-prompt-form'>{props.children}</div>
      <div class='pages-prompt-buttons'>
        <div
          classList={{
            'pages-prompt-button': true,
            'ok': true,
            'disabled': props.disabled?.() === true,
          }}
          onClick={() => {
            if (props.disabled?.() !== true) {
              props.ok();
            }
          }}
        >
          <icon.Ok />
        </div>
        <div class='pages-prompt-button cancel' onClick={props.cancel}>
          <icon.Cancel />
        </div>
      </div>
    </div>
  </div>
);

export type Props = {
  hide: () => void;
};
