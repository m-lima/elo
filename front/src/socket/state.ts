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

export type Listener = (state: State) => void;

export class Manager {
  private state: State;
  private readonly listeners: Listener[];

  public constructor(state: State) {
    this.state = state;
    this.listeners = [];
  }

  public get() {
    return this.state;
  }

  public set(state: State) {
    if (state === this.state) {
      this.state = state;
      for (const listener of this.listeners) {
        listener(this.state);
      }
    }
  }

  public isDisconnected() {
    return isDisconnected(this.state);
  }

  public isError() {
    return this.state === Disconnected.Error;
  }

  public isFetching() {
    return this.state === Connected.Fetching;
  }

  public isUnauthorized() {
    return this.state === Disconnected.Unauthorized;
  }

  public registerListener(listener: Listener) {
    this.listeners.push(listener);
    return listener;
  }

  public unregisterListener(listener: Listener) {
    const index = this.listeners.indexOf(listener);
    if (index >= 0) {
      this.listeners.splice(index, 1);
    }
  }
}
