import { encode, decode } from '@msgpack/msgpack';

export class Timeout {
  private readonly millis: number;

  public constructor(millis: number) {
    this.millis = millis;
  }

  public getMillis() {
    return this.millis;
  }
}

export class SocketNotConnected {
  private readonly state: DisconnectedState;

  public constructor(state: DisconnectedState) {
    this.state = state;
  }

  public getState() {
    return this.state;
  }

  public toSring() {
    switch (this.state) {
      case DisconnectedState.Connecting:
        return 'Connecting';
      case DisconnectedState.Closed:
        return 'Closed socket';
      case DisconnectedState.Error:
        return 'Socket error';
      case DisconnectedState.Unauthorized:
        return 'Unauthorized';
    }
  }
}

type SocketState = DisconnectedState | ConnectedState;

const isDisconnectedState = (state: SocketState) =>
  state === DisconnectedState.Connecting ||
  state === DisconnectedState.Closed ||
  DisconnectedState.Error ||
  DisconnectedState.Unauthorized;

export enum DisconnectedState {
  Connecting = 0,
  Closed = 1,
  Error = 2,
  Unauthorized = 3,
}

export enum ConnectedState {
  Open = 4,
  Ready = 5,
  Fetching = 6,
}

type SocketStateListener = (state: SocketState) => void;
type Handler<Message> = (message: Message) => boolean;
type RequestHandler<Message, Response> = (message: Message) => Response | undefined;

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
      reject(new Timeout(timeout));
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

  public abort(socketNotConnected: SocketNotConnected) {
    this.reject(socketNotConnected);
  }
}

export class Socket<Message, Response> {
  private readonly requests: RequestHandlerInner<Message, Response>[];
  private readonly handlers: Handler<Message>[];
  private readonly stateListeners: SocketStateListener[];

  private socket: WebSocket;
  private state: SocketState;
  private attempts: number;

  public constructor(url: string | URL, checkUrl?: string | URL) {
    this.requests = [];
    this.handlers = [];
    this.stateListeners = [];

    this.state = DisconnectedState.Closed;
    this.attempts = 0;
    this.socket = this.connect(url, checkUrl);
  }

  private connect(url: string | URL, checkUrl?: string | URL) {
    this.setState(DisconnectedState.Connecting);

    const socket = new WebSocket(url);

    socket.onerror = () => {
      // Check only in the first failure
      if (this.attempts === 0 && !!checkUrl) {
        void fetch(checkUrl, { credentials: 'include', redirect: 'manual' }).then(r => {
          if ((r.status >= 300 && r.status < 400) || r.status === 401 || r.status === 403) {
            this.setState(DisconnectedState.Unauthorized);
          }
        });
      }

      this.setState(DisconnectedState.Error);
    };

    socket.onclose = () => {
      if (this.state !== DisconnectedState.Error) {
        this.setState(DisconnectedState.Closed);
      }

      this.tryReconnect(url, checkUrl);
    };

    socket.onopen = () => {
      this.attempts = 0;
      this.setState(ConnectedState.Open);
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
    if (this.state === DisconnectedState.Unauthorized) {
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

  private setState(state: SocketState) {
    if (this.state !== state) {
      // Unauthorized can only be overriden by ConnectedState
      if (this.state === DisconnectedState.Unauthorized && isDisconnectedState(state)) {
        return;
      }

      if (isDisconnectedState(state)) {
        this.requests.forEach(r => {
          r.abort(new SocketNotConnected(state as DisconnectedState));
        });
      }

      this.state = state;

      for (const listener of this.stateListeners) {
        listener(this.state);
      }
    }
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

  public request<Request>(
    request: Request,
    handler: RequestHandler<Message, Response>,
    timeout: number = 30000,
  ): Promise<Response> {
    if (isDisconnectedState(this.state)) {
      return Promise.reject(new SocketNotConnected(this.state as DisconnectedState));
    }

    // TODO: maybe queue requests if the socket is not ready
    const payload = encode(request);
    let requestInstance: RequestHandlerInner<Message, Response>;
    return new Promise<Response>((accept, reject) => {
      requestInstance = new RequestHandlerInner(handler, accept, reject, timeout);
      this.requests.push(requestInstance);
      this.setState(ConnectedState.Fetching);
      this.socket.send(payload);
    }).finally(() => {
      const index = this.requests.indexOf(requestInstance);
      if (index >= 0) {
        this.requests.splice(index, 1);
        if (this.requests.length === 0 && this.state === ConnectedState.Fetching) {
          this.setState(ConnectedState.Ready);
        }
      }
    });
  }

  public getState() {
    return this.state;
  }

  public registerStateListener(listener: SocketStateListener) {
    this.stateListeners.push(listener);
    return listener;
  }

  public unregisterStateListener(listener: SocketStateListener) {
    const index = this.stateListeners.indexOf(listener);
    if (index >= 0) {
      this.stateListeners.splice(index, 1);
    }
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
