import './spinner.css';
import spinner from '../assets/spinner.svg';

const Inner = () => <img src={spinner} class='components_spinner' />;

export const Spinner = (props: { size?: string }) =>
  props.size
    ? <div
      style={{
        width: props.size,
        height: props.size,
      }}
    >
      <Inner />
    </div>
    : <Inner />;
