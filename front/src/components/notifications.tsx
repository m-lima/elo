import { For, createEffect, createSignal, onCleanup } from 'solid-js';
import { useStore } from '../store';

import './notifications.css';

export const Notifications = () => {
  const store = useStore();
  const [notifications, setNotifications] = createSignal<Notification[]>([], { equals: false });
  const [fades, setFades] = createSignal<Fade[]>([], { equals: false });

  createEffect(() => {
    const handler = (message: string) => {
      const id = Math.random();
      const notification = { id, message };
      const fade = { id, fade: false };
      setNotifications(list => {
        setTimeout(() => {
          console.debug('Message timeout');
          setFades(list => {
            const idx = list.findIndex(i => i.id === id);
            if (idx >= 0) {
              console.debug('Fading');
              list[idx].fade = true;
            }
            return list;
          });
          setTimeout(() => {
            setNotifications(list => {
              const idx = list.findIndex(i => i.id === id);
              if (idx >= 0) {
                console.debug('Removing');
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

    console.debug('Subscribing');
    store.subscribe(handler);
    onCleanup(() => {
      console.debug('Unsubscribing');
      store.unsubscribe(handler);
    });
  });

  return (
    <div class='components-notifications'>
      <For each={notifications()}>
        {(n, i) => (
          <div
            classList={{ 'components-notifications-message': true, 'fading': fades()[i()]?.fade }}
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
};

type Fade = {
  id: number;
  fade: boolean;
};
