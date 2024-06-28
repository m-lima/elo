import { icon } from '../components';

import './error.css';

export const Unauthorized = (props: { id?: string }) => (
  <div class='page-error' id={props.id}>
    <icon.Fingerprint />
    <h1>Unauthorized</h1>
  </div>
);

export const TimeOut = (props: { id?: string }) => (
  <div class='page-error' id={props.id}>
    <icon.Timeout />
    <h1>Timeed out</h1>
  </div>
);

export const GenericError = (props: { id?: string }) => (
  <div class='page-error' id={props.id}>
    <icon.SadFace />
    <h1>Something went wrong</h1>
  </div>
);

export const NotFound = (props: { id?: string }) => (
  <div class='page-error' id={props.id}>
    <icon.Magnifier />
    <h1>Not found</h1>
  </div>
);
