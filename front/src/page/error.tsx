import { icon } from '../components';

import './error.css';

export const Unauthorized = () => {
  return (
    <div class='components-error'>
      <icon.Fingerprint />
      <h1>Unauthorized</h1>
    </div>
  );
};

export const TimeOut = () => {
  return (
    <div class='components-error'>
      <icon.Timeout />
      <h1>Timeed out</h1>
    </div>
  );
};

export const GenericError = () => {
  return (
    <div class='components-error'>
      <icon.SadFace />
      <h1>Something went wrong</h1>
    </div>
  );
};

export const NotFound = () => {
  return (
    <div class='components-error'>
      <icon.Magnifier />
      <h1>Not found</h1>
    </div>
  );
};
