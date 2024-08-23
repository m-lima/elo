import type { Message, Ok, OkResponse } from './message';

export const newRequestId = () => Math.floor(Math.random() * 1024 * 1024);

export class ResponseError extends Error {}

export class FetchError extends Error {
  constructor(id: number, code: number, message?: string) {
    super(
      message !== undefined
        ? `Error fetching request [id: ${id} code: ${code}]: ${message}`
        : `Error fetching request [id: ${id} code: ${code}]`,
    );
  }
}

export const preValidateMessage = (id: number, message: Message): Ok | undefined => {
  if ('push' in message) {
    return;
  }

  if ('error' in message) {
    if (message.id !== id) {
      return;
    }
    throw new ResponseError(message.error.message);
  }

  return message.ok;
};

export const validateMessage = <F extends keyof OkResponse>(
  id: number,
  field: F,
  message: Message,
): Pick<OkResponse, F> | undefined => {
  const validated = preValidateMessage(id, message);

  if (validated === undefined) {
    return;
  }

  if ('done' === validated) {
    throw new FetchError(id, -400, `Did not receive a '${field}' response`);
  }

  if (field in validated) {
    return validated;
  }
  throw new FetchError(id, -400, `Did not receive a '${field}' response`);
};

export const validateDone = (id: number, message: Message): true | undefined => {
  const validated = preValidateMessage(id, message);

  if (validated === undefined) {
    return;
  }

  if ('done' === validated) {
    return true;
  } else {
    throw new FetchError(id, -400, `Did not receive a 'done' response`);
  }
};
