import './spinner.css';

const Inner = () => <span class='components spinner' />;
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
