import type { Message, Ok, OkResponse } from './message';

export const newRequestId = () => Math.floor(Math.random() * 1024 * 1024);

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
    throw new FetchError(id, message.error.code, message.error.message);
  }

  return message.ok;
};

export const validateMessage = <F extends keyof OkResponse>(
  id: number,
  field: F,
  message: Message,
): OkResponse[F] | undefined => {
  const validated = preValidateMessage(id, message);

  if (validated === undefined) {
    return;
  }

  if (validated === 'done') {
    throw new FetchError(id, -400, `Did not receive a '${field}' field`);
  }

  if (field in validated) {
    return validated[field];
  } else {
    throw new FetchError(id, -400, `Did not receive a '${field}' field`);
  }
};

export const validateMessages = <F extends keyof OkResponse>(
  id: number,
  fields: F[],
  message: Message,
): Pick<OkResponse, F> | undefined => {
  const validated = preValidateMessage(id, message);

  if (validated === undefined) {
    return;
  }

  if (validated === 'done') {
    throw new FetchError(id, -400, `Did not receive any of '${JSON.stringify(fields)}' fields`);
  }

  for (const field of fields) {
    if (field in validated) {
      return validated;
    }
  }
  throw new FetchError(id, -400, `Did not receive any of '${JSON.stringify(fields)}' fields`);
};

export const validateDone = (id: number, message: Message): true | undefined => {
  if ('push' in message) {
    return;
  }

  if ('error' in message) {
    if (message.id !== id) {
      return;
    }
    throw new FetchError(id, message.error.code, message.error.message);
  }

  if ('ok' in message) {
    return true;
  } else {
    throw new FetchError(id, -400, `Did not receive a 'done' response`);
  }
};
