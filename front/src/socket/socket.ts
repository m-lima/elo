import { state } from '.';

export interface Socket<Request, Message, Response> {
  request(
    request: Request,
    handler: RequestHandler<Message, Response>,
    timeout?: number,
  ): Promise<Response>;

  getState(): state.State;

  registerStateListener(listener: state.Listener): state.Listener;
  unregisterStateListener(listener: state.Listener): void;

  registerHandler(handler: Handler<Message>): Handler<Message>;
  unregisterHandler(handler: Handler<Message>): void;
}

export type Handler<Message> = (message: Message) => boolean;
export type RequestHandler<Message, Response> = (message: Message) => Response | undefined;
