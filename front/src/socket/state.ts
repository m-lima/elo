export type State = Connected | Disconnected;

export enum Disconnected {
  Connecting = 0,
  Closed = 1,
  Error = 2,
  Unauthorized = 3,
}

export enum Connected {
  Open = 4,
  Ready = 5,
  Fetching = 6,
}

export const isDisconnected = (state: State) =>
  state === Disconnected.Connecting ||
  state === Disconnected.Closed ||
  state === Disconnected.Error ||
  state === Disconnected.Unauthorized;

export const isConnected = (state: State) =>
  state === Connected.Open || state === Connected.Ready || state === Connected.Fetching;

export type Listener = (state: State) => void;
