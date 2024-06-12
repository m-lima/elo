/// TODO: EXCLUDE(PROD)

import { state, type RequestHandler, type Socket, type Handler } from '.';

export class Mock<Request, Message, Response> implements Socket<Request, Message, Response> {
  private readonly handlers: Handler<Message>[];
  private readonly state: state.Manager;

  public constructor() {
    this.handlers = [];

    const s = new state.Manager(state.Disconnected.Closed);
    this.state = s;

    setTimeout(() => {
      s.set(state.Disconnected.Connecting);
      setTimeout(() => {
        s.set(state.Connected.Open);
      }, 750);
    }, 750);
  }

  public request(
    _request: Request,
    _handler: RequestHandler<Message, Response>,
    _timeout?: number | undefined,
  ): Promise<Response> {
    throw new Error('Method not implemented.');
  }

  public getState(): state.State {
    return this.state.get();
  }

  public registerStateListener(listener: state.Listener): state.Listener {
    return this.state.registerListener(listener);
  }

  public unregisterStateListener(listener: state.Listener): void {
    this.state.unregisterListener(listener);
  }

  public registerHandler(handler: Handler<Message>): Handler<Message> {
    this.handlers.push(handler);
    return handler;
  }

  public unregisterHandler(handler: Handler<Message>): void {
    const index = this.handlers.indexOf(handler);
    if (index >= 0) {
      this.handlers.splice(index, 1);
    }
  }
}
