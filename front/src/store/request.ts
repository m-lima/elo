import type { Message, Ok } from './message';

export const newRequestId = () => Math.floor(Math.random() * 1024 * 1024);

class FetchError extends Error {
  constructor(id: number, code: number, message?: string) {
    super(
      message !== undefined
        ? `Error fetching request [id: ${id} code: ${code}]: ${message}`
        : `Error fetching request [id: ${id} code: ${code}]`,
    );
  }
}

export const validateMessage = <F extends keyof Ok>(
  id: number,
  field: F,
  message: Message,
): Ok[F] | undefined => {
  if ('push' in message) {
    return;
  }

  if ('error' in message) {
    if (message.id !== id) {
      return;
    }
    throw new FetchError(id, message.error.code, message.error.message);
  }

  if (field in message.ok) {
    return message.ok[field];
  } else {
    throw new FetchError(id, -400, `Did not receive a '${field}' field`);
  }
};
