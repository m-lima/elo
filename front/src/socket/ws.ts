import { encode, decode } from '@msgpack/msgpack';

import { state, error, type RequestHandler, type Socket, type Handler } from '.';

type Accept<Response> = (response: Response | PromiseLike<Response>) => void;
// Allowed to match the Promise signature
/* eslint-disable-next-line
   @typescript-eslint/no-explicit-any
*/
type Reject = (reason?: any) => void;

class RequestHandlerInner<Message, Response> {
  readonly handler: RequestHandler<Message, Response>;
  readonly accept: Accept<Response>;
  readonly reject: Reject;

  public constructor(
    handler: RequestHandler<Message, Response>,
    accept: Accept<Response>,
    reject: Reject,
    timeout: number,
  ) {
    this.handler = handler;
    this.accept = accept;
    this.reject = reject;

    setTimeout(() => {
      reject(new error.Timeout(timeout));
    }, timeout);
  }

  public handle(message: Message): boolean {
    try {
      const response = this.handler(message);

      if (response !== undefined) {
        this.accept(response);
        return true;
      } else {
        return false;
      }
    } catch (error) {
      this.reject(error);
      return true;
    }
  }

  public abort(disconnected: error.Disconnected) {
    this.reject(disconnected);
  }
}

export class Ws<Request, Message, Response> implements Socket<Request, Message, Response> {
  private readonly requests: RequestHandlerInner<Message, Response>[];
  private readonly handlers: Handler<Message>[];
  private readonly state: state.Manager;

  private socket: WebSocket;
  private attempts: number;

  public constructor(url: string | URL, checkUrl?: string | URL) {
    this.requests = [];
    this.handlers = [];
    this.state = new state.Manager(state.Disconnected.Closed);

    this.attempts = 0;
    this.socket = this.connect(url, checkUrl);
  }

  private connect(url: string | URL, checkUrl?: string | URL) {
    this.setState(state.Disconnected.Connecting);

    const socket = new WebSocket(url);

    socket.onerror = () => {
      // Check only in the first failure
      if (this.attempts === 0 && !!checkUrl) {
        void fetch(checkUrl, { credentials: 'include', redirect: 'manual' }).then(r => {
          if ((r.status >= 300 && r.status < 400) || r.status === 401 || r.status === 403) {
            this.setState(state.Disconnected.Unauthorized);
          }
        });
      }

      this.setState(state.Disconnected.Error);
    };

    socket.onclose = () => {
      if (this.state.isError()) {
        this.setState(state.Disconnected.Closed);
      }

      this.tryReconnect(url, checkUrl);
    };

    socket.onopen = () => {
      this.attempts = 0;
      this.setState(state.Connected.Open);
    };

    socket.onmessage = evt => {
      this.onMessage(evt);
    };

    socket.binaryType = 'arraybuffer';

    this.socket = socket;
    return socket;
  }

  private nextAttempt() {
    // Unauthorized is always fatal
    if (this.state.isUnauthorized()) {
      return;
    }

    switch (this.attempts) {
      case 0:
        return 0;
      case 1:
        return 5 * 1000;
      case 2:
        return 10 * 1000;
      case 3:
        return 15 * 1000;
      default:
        return;
    }
  }

  private tryReconnect(url: string | URL, checkUrl?: string | URL) {
    const timeout = this.nextAttempt();

    if (timeout === undefined) {
      return;
    }

    this.attempts += 1;
    setTimeout(() => this.connect(url, checkUrl), timeout);
  }

  private setState(newState: state.State) {
    if (state.isDisconnected(newState)) {
      // Unauthorized can only be overriden by ConnectedState
      if (this.state.isUnauthorized()) {
        return;
      }

      this.requests.forEach(r => {
        r.abort(new error.Disconnected(newState as state.Disconnected));
      });
    }

    this.state.set(newState);
  }

  private onMessage(e: MessageEvent) {
    if (!(e.data instanceof ArrayBuffer)) {
      console.error('Received a text message on a binary channel:');
      console.error(e.data);
    }

    const message = decode(e.data as ArrayBuffer) as Message;

    for (let i = 0; i < this.requests.length; ++i) {
      if (this.requests[i].handle(message)) {
        return;
      }
    }

    for (let i = 0; i < this.handlers.length; ++i) {
      if (this.handlers[i](message)) {
        return;
      }
    }

    console.log('Unprocessed message received', JSON.stringify(message));
  }

  public request(
    request: Request,
    handler: RequestHandler<Message, Response>,
    timeout: number = 30000,
  ): Promise<Response> {
    if (this.state.isDisconnected()) {
      return Promise.reject(new error.Disconnected(this.state.get() as state.Disconnected));
    }

    // TODO: maybe queue requests if the socket is not ready
    const payload = encode(request);
    let requestInstance: RequestHandlerInner<Message, Response>;
    return new Promise<Response>((accept, reject) => {
      requestInstance = new RequestHandlerInner(handler, accept, reject, timeout);
      this.requests.push(requestInstance);
      this.setState(state.Connected.Fetching);
      this.socket.send(payload);
    }).finally(() => {
      const index = this.requests.indexOf(requestInstance);
      if (index >= 0) {
        this.requests.splice(index, 1);
        if (this.requests.length === 0 && this.state.isFetching()) {
          this.setState(state.Connected.Ready);
        }
      }
    });
  }

  public getState() {
    return this.state.get();
  }

  public registerStateListener(listener: state.Listener) {
    return this.state.registerListener(listener);
  }

  public unregisterStateListener(listener: state.Listener) {
    this.state.unregisterListener(listener);
  }

  public registerHandler(handler: Handler<Message>) {
    this.handlers.push(handler);
    return handler;
  }

  public unregisterHandler(handler: Handler<Message>) {
    const index = this.handlers.indexOf(handler);
    if (index >= 0) {
      this.handlers.splice(index, 1);
    }
  }
}
