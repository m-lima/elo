import { icon } from '../components';
import * as consts from '../consts';

import './error.css';

export const Unauthorized = (props: { inline?: boolean }) => (
  <div class='pages-error' id={props.inline === false ? undefined : 'main'}>
    <icon.Fingerprint />
    <h1>Unauthorized</h1>
  </div>
);

export const TimeOut = (props: { inline?: boolean }) => (
  <div class='pages-error' id={props.inline === false ? undefined : 'main'}>
    <icon.Timeout />
    <h1>Timeed out</h1>
  </div>
);

export const GenericError = (props: { inline?: boolean }) => (
  <div class='pages-error' id={props.inline === false ? undefined : 'main'}>
    <icon.SadFace />
    <h1>Something went wrong</h1>
  </div>
);

export const NotFound = (props: { inline?: boolean }) => (
  <div class='pages-error' id={props.inline === false ? undefined : 'main'}>
    <icon.Magnifier />
    <h1>Not found</h1>
  </div>
);

export const NoGames = (props: { inline?: boolean }) => (
  <div class='pages-error' id={props.inline === false ? undefined : 'main'}>
    <icon.Wink />
    <h1>No games yet</h1>
  </div>
);

export const Version = (props: { inline?: boolean }) => (
  <div class='pages-error' id={props.inline === false ? undefined : 'main'}>
    <icon.Version />
    <h1>Version mismatch</h1>
    <h2>
      The server is running a newer version of <b>{consts.name}</b>
    </h2>
    <p>Please reload the page</p>
  </div>
);
