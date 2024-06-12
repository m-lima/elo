import { icon } from '.';

import './status.css';

export const Connecting = () => {
  return (
    <div class='components-status'>
      <icon.Spinner /> Connecting
    </div>
  );
};

export const Loading = () => {
  return (
    <div class='components-status'>
      <icon.Spinner /> Loading
    </div>
  );
};
