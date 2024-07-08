import { For, createEffect, createSignal, onCleanup } from 'solid-js';
import { useStore } from '../store';

import './notifications.css';

export const Notifications = () => {
  const store = useStore();
  const [notifications, setNotifications] = createSignal<Notification[]>([], { equals: false });
  const [fades, setFades] = createSignal<Fade[]>([], { equals: false });

  createEffect(() => {
    const handler = (message: string, error: boolean) => {
      const id = Math.random();
      const notification = { id, message, error };
      const fade = { id, fade: false };
      setNotifications(list => {
        setTimeout(() => {
          setFades(list => {
            const idx = list.findIndex(i => i.id === id);
            if (idx >= 0) {
              list[idx].fade = true;
            }
            return list;
          });
          setTimeout(() => {
            setNotifications(list => {
              const idx = list.findIndex(i => i.id === id);
              if (idx >= 0) {
                list.splice(idx, 1);
              }
              return list;
            });
          }, 500);
        }, 5000);
        return [notification, ...list];
      });
      setFades(list => [fade, ...list]);
    };

    store.subscribe(handler);
    onCleanup(() => {
      store.unsubscribe(handler);
    });
  });

  return (
    <div class='components-notifications'>
      <For each={notifications()}>
        {(n, i) => (
          <div
            classList={{
              'components-notifications-message': true,
              'error': n.error,
              'fading': fades()[i()]?.fade,
            }}
          >
            {n.message}
          </div>
        )}
      </For>
    </div>
  );
};

type Notification = {
  id: number;
  message: string;
  error: boolean;
};

type Fade = {
  id: number;
  fade: boolean;
};
