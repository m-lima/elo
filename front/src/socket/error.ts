import { state } from '.';

export class Timeout extends Error {
  private readonly millis: number;

  public constructor(millis: number) {
    super('Timeout');
    this.millis = millis;
  }

  public getMillis() {
    return this.millis;
  }
}

export class Disconnected extends Error {
  private readonly state: state.Disconnected;

  public constructor(state: state.Disconnected) {
    super(disconnectedStateToString(state));
    this.state = state;
  }

  public getState() {
    return this.state;
  }
}

const disconnectedStateToString = (s: state.Disconnected) => {
  switch (s) {
    case state.Disconnected.Connecting:
      return 'Connecting';
    case state.Disconnected.Closed:
      return 'Closed socket';
    case state.Disconnected.Error:
      return 'Socket error';
    case state.Disconnected.Unauthorized:
      return 'Unauthorized';
  }
};
