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

export const isPending = (state: State) =>
  state === Disconnected.Connecting || state === Connected.Fetching;

export type Listener = (state: State) => void;

export const toString = (state: State) => {
  switch (state) {
    case Disconnected.Connecting:
      return 'Connecting';
    case Disconnected.Closed:
      return 'Closed';
    case Disconnected.Error:
      return 'Error';
    case Disconnected.Unauthorized:
      return 'Unauthorized';
    case Connected.Open:
      return 'Open';
    case Connected.Ready:
      return 'Ready';
    case Connected.Fetching:
      return 'Fetching';
    default:
      return 'Unknown';
  }
};
