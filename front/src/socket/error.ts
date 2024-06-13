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

  public constructor(reason: state.Disconnected) {
    super(`Socket disconnected: ${state.toString(reason)}`);
    this.state = reason;
  }

  public getState() {
    return this.state;
  }
}
