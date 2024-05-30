import { DEV } from "solid-js";

import { Socket } from "../socket";

export class Store {
  private readonly socket: Backend;

  public constructor(url: string | URL | undefined) {
    this.socket = new Socket(url);
  }
}

export interface Backend {

}
