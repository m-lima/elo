import { icon } from '../components';

import './error.css';

export const Unauthorized = () => (
  <div class='pages-error' id='main'>
    <icon.Fingerprint />
    <h1>Unauthorized</h1>
  </div>
);

export const TimeOut = () => (
  <div class='pages-error' id='main'>
    <icon.Timeout />
    <h1>Timeed out</h1>
  </div>
);

export const GenericError = () => (
  <div class='pages-error' id='main'>
    <icon.SadFace />
    <h1>Something went wrong</h1>
  </div>
);

export const NotFound = () => (
  <div class='pages-error' id='main'>
    <icon.Magnifier />
    <h1>Not found</h1>
  </div>
);
