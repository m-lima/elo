export const setCookie = (name: string, value: string, expirationDays: number = 365) => {
  const expiry = new Date(new Date().getTime() + expirationDays * 24 * 60 * 60 * 100).toUTCString();
  document.cookie = `${name}=${value};expires=${expiry};path=/;SameSite=Lax;Secure`;
};

export const unsetCookie = (name: string) => {
  document.cookie = `${name}=;expires=${new Date().toUTCString()};path=/;SameSite=Lax;Secure`;
};

export const getCookie = (name: string) => {
  const prefix = `${name}=`;
  return document.cookie
    .split(';')
    .find(p => p.trim().startsWith(prefix))
    ?.substring(prefix.length)
    .trim();
};

export const date = {
  monthToString: (month: number) => {
    switch (month) {
      case 0:
        return 'Jan';
      case 1:
        return 'Feb';
      case 2:
        return 'Mar';
      case 3:
        return 'Apr';
      case 4:
        return 'May';
      case 5:
        return 'Jun';
      case 6:
        return 'Jul';
      case 7:
        return 'Aug';
      case 8:
        return 'Sep';
      case 9:
        return 'Oct';
      case 10:
        return 'Nov';
      case 11:
        return 'Dec';
    }
  },

  toString: (d: Date) =>
    `${String(d.getDate()).padStart(2, '0')}-${date.monthToString(d.getMonth())} ${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`,

  toShortString: (d: Date) =>
    `${String(d.getDate()).padStart(2, '0')}/${date.monthToString(d.getMonth())}/${String(d.getFullYear() % 1000).padStart(2, '0')} `,

  toLongString: (d: Date) =>
    `${String(d.getDate()).padStart(2, '0')}/${date.monthToString(d.getMonth())}/${d.getFullYear()} ${String(d.getHours()).padStart(2, '0')}:${String(d.getMinutes()).padStart(2, '0')}`,
};

export class Maybe<T> {
  private readonly value?: T;

  constructor(value?: T) {
    this.value = value;
  }

  static from<U>(value?: U) {
    return new Maybe(value);
  }

  public map<R>(action: (value: T) => R | undefined) {
    if (this.value === undefined) {
      return new Maybe<R>();
    }

    return new Maybe(action(this.value));
  }

  public then<R>(action: (value: T) => R | undefined) {
    if (this.value === undefined) {
      return;
    }

    return action(this.value);
  }

  public else(value: T) {
    if (this.value === undefined) {
      return value;
    } else {
      return this.value;
    }
  }
}
