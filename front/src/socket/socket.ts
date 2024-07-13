import { Encoder, Decoder } from '@msgpack/msgpack';

import { state, error } from '.';

type Accept<Response> = (response: Response | PromiseLike<Response>) => void;
// Allowed to match the Promise signature
// eslint-disable-next-line @typescript-eslint/no-explicit-any
type Reject = (reason?: any) => void;
type Handler<Message> = (message: Message) => boolean;
type RequestHandler<Message, Response> = (message: Message) => Response | undefined;

interface RequestHandlerInner<Message> {
  handle(message: Message): boolean;
  abort(disconnected: error.Disconnected): void;
}

class RequestHandlerInnerImpl<Message, Response> implements RequestHandlerInner<Message> {
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

export class Socket<Request, Message> {
  private readonly requests: RequestHandlerInner<Message>[];
  private readonly awaitingRequests: Accept<boolean>[];
  private readonly handlers: Handler<Message>[];
  private readonly stateListeners: state.Listener[];
  private readonly encoder: Encoder;
  private readonly decoder: Decoder;

  private socket: WebSocket;
  private state: state.State;
  private attempts: number;

  public constructor(url: string | URL, checkUrl?: string | URL, loginUrl?: string | URL) {
    this.requests = [];
    this.awaitingRequests = [];
    this.handlers = [];
    this.stateListeners = [];
    this.encoder = new Encoder();
    this.decoder = new Decoder();

    this.state = state.Disconnected.Closed;
    this.attempts = 0;
    this.socket = this.connect(url, checkUrl, loginUrl);
  }

  private connect(url: string | URL, checkUrl?: string | URL, loginUrl?: string | URL) {
    this.setState(state.Disconnected.Connecting);

    const socket = new WebSocket(url);

    socket.onerror = () => {
      // Check only in the first failure
      if (this.attempts === 0 && checkUrl !== undefined) {
        void fetch(checkUrl, { credentials: 'include', redirect: 'manual' }).then(r => {
          if ((r.status >= 300 && r.status < 400) || r.status === 401) {
            if (loginUrl !== undefined) {
              if (typeof loginUrl === 'string') {
                window.location.href = loginUrl;
              } else {
                window.location.href = loginUrl.href;
              }
            } else {
              this.setState(state.Disconnected.Unauthorized);
            }
          } else if (r.status === 403) {
            this.setState(state.Disconnected.Unauthorized);
          }
        });
      }

      this.setState(state.Disconnected.Error);
    };

    socket.onclose = () => {
      if (this.state !== state.Disconnected.Error) {
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
    this.setState(state.Disconnected.Connecting);
    setTimeout(() => {
      // Unauthorized is always fatal
      if (this.state !== state.Disconnected.Unauthorized) {
        this.connect(url, checkUrl);
      }
    }, timeout);
  }

  private setState(newState: state.State) {
    if (this.state !== newState) {
      if (state.isDisconnected(newState)) {
        // Unauthorized can only be overriden by ConnectedState
        if (this.state === state.Disconnected.Unauthorized) {
          return;
        }

        this.requests.forEach(r => {
          r.abort(new error.Disconnected(newState as state.Disconnected));
        });
      }

      this.state = newState;

      for (const listener of this.stateListeners) {
        listener(this.state);
      }

      if (newState === state.Connected.Open) {
        for (const awaitingRequest of this.awaitingRequests) {
          awaitingRequest(false);
        }
        this.awaitingRequests.splice(0, this.awaitingRequests.length);
      }
    }
  }

  private onMessage(e: MessageEvent) {
    if (!(e.data instanceof ArrayBuffer)) {
      console.error('Received a text message on a binary channel:');
      console.error(e.data);
    }

    const message = this.decoder.decode(e.data as ArrayBuffer) as Message;

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

    console.error('Unprocessed message received', JSON.stringify(message));
  }

  public async request<Response>(
    request: Request,
    handler: RequestHandler<Message, Response>,
    timeout: number = 30000,
    connectionTimeout: number = 30000,
  ): Promise<Response> {
    if (this.state === state.Disconnected.Connecting) {
      const timedOut = await new Promise<boolean>(accept => {
        this.awaitingRequests.push(accept);
        setTimeout(() => {
          const index = this.awaitingRequests.indexOf(accept);
          if (index >= 0) {
            this.awaitingRequests.splice(index, 1);
          }
          accept(true);
        }, connectionTimeout);
      });

      if (timedOut) {
        return Promise.reject(new error.Timeout(connectionTimeout));
      }
    }

    const payload = this.encoder.encode(request);
    let requestInstance: RequestHandlerInnerImpl<Message, Response>;
    return new Promise<Response>((accept, reject) => {
      requestInstance = new RequestHandlerInnerImpl(handler, accept, reject, timeout);
      this.requests.push(requestInstance);
      this.setState(state.Connected.Fetching);
      this.socket.send(payload);
    }).finally(() => {
      const index = this.requests.indexOf(requestInstance);
      if (index >= 0) {
        this.requests.splice(index, 1);
        if (this.requests.length === 0 && this.state === state.Connected.Fetching) {
          this.setState(state.Connected.Ready);
        }
      }
    });
  }

  public getState() {
    return this.state;
  }

  public registerStateListener(listener: state.Listener) {
    this.stateListeners.push(listener);
    return listener;
  }

  public unregisterStateListener(listener: state.Listener) {
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
