import './icon.css';

type Props = {
  src: string,
  size?: string,
};

export const Icon = (props: Props) =>
  <img
    src={props.src}
    class='components_icon'
  />;
