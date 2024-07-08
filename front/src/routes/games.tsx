import { Show, Suspense, createMemo, createSignal } from 'solid-js';

import { useStore } from '../store';
import { icon, prompt, Action, Games as GameTable, Actions } from '../components';
import { Loading, Main } from '../pages';

export const Games = () => {
  const store = useStore();
  const self = store.getSelf();
  const games = store.getGames();
  const players = store.getPlayers();
  const [promptVisible, setPromptVisible] = createSignal(false);
  const shown = createMemo(prev => {
    if (prev === true) {
      return true;
    } else {
      return promptVisible();
    }
  });
  const opponents = createMemo(() => {
    if (!shown()) {
      return [];
    }

    const innerSelf = self();
    const innerGames = games();
    const innerPlayers = players();

    if (innerSelf === undefined || innerGames === undefined || innerPlayers === undefined) {
      return [];
    }

    return Array.from(
      innerGames
        .map(g => {
          if (g.playerOne === innerSelf.id) {
            return g.playerTwo;
          } else if (g.playerTwo === innerSelf.id) {
            return g.playerOne;
          } else {
            return;
          }
        })
        .reduce(
          (acc, curr) => {
            if (curr !== undefined) {
              const entry = acc.get(curr);
              if (entry !== undefined) {
                acc.set(curr, { name: entry.name, count: entry.count + 1 });
              }
            }
            return acc;
          },
          new Map(
            innerPlayers
              .filter(p => p.id !== innerSelf.id)
              .map(p => [p.id, { name: p.name, count: 0 }]),
          ),
        )
        .entries(),
    )
      .sort((a, b) => b[1].count - a[1].count)
      .map(o => {
        return { id: o[0], name: o[1].name };
      });
  });

  return (
    <Suspense fallback=<Loading />>
      <Show when={promptVisible()}>
        <prompt.Game
          hide={() => setPromptVisible(false)}
          store={store}
          self={() => players()?.find(p => p.id === self()?.id)}
          opponents={opponents}
        />
      </Show>
      <Actions>
        <Action
          icon=<icon.Add />
          text='New game'
          action={() => {
            setPromptVisible(true);
          }}
        />
      </Actions>
      <Main>
        <GameTable players={players} games={games} />
      </Main>
    </Suspense>
  );
};
