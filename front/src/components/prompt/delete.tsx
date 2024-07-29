import { createSignal } from 'solid-js';

import { Game } from '../../types';
import { Prompt, type Props } from './prompt';
import { Store } from '../../store';

export const Delete = (
  props: Props & {
    store: Store;
    game: () => Game;
  },
) => {
  const [busy, setBusy] = createSignal<boolean | undefined>();

  const commit = () => {
    const game = props.game();

    setTimeout(() => setBusy(busy => busy ?? true), 200);
    props.store
      .editGame({ ...game, deleted: !game.deleted })
      .then(r => {
        if (r) {
          props.hide();
        }
      })
      .finally(() => {
        setBusy(false);
        setTimeout(setBusy, 500);
      });
  };

  return (
    <Prompt
      visible={props.visible}
      ok={commit}
      cancel={props.hide}
      disabled={() => false}
      busy={busy}
    />
  );
};
