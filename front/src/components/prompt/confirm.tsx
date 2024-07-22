import { Prompt, type Props } from './prompt';

import './rename.css';

export const Confirm = (
  props: Props & {
    action: () => void;
  },
) => {
  return <Prompt visible={props.visible} ok={props.action} cancel={props.hide} />;
};
