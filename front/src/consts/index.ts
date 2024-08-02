export const name = 'EloPong';

export const version = 2;

export const host = {
  ws:
    import.meta.env.VITE_HOST_WS === undefined
      ? `ws://${location.hostname}:3333/ws/binary`
      : import.meta.env.VITE_HOST_WS,
  check:
    import.meta.env.VITE_HOST_CHECK === undefined
      ? `http://${location.hostname}:3333/check`
      : import.meta.env.VITE_HOST_CHECK,
  login: import.meta.env.VITE_HOST_LOGIN,
};

export const colors = {
  accent: '#ffa500',
  accentSemiTransparent: '#ffa50080',
  red: '#a03030',
  redSemiTransparent: '#a0303080',
  green: '#30a030',
  greenSemiTransparent: '#30a03080',
};

export const limit = {
  gameList: 100,
  chart: 50,
};
