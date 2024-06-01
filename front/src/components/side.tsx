import { A } from "@solidjs/router";
import { Suspense } from "solid-js";

import trophy from '../assets/trophy.svg';
import user from '../assets/user.svg';
import { useSelf } from "../store";

import './side.css';

export const Side = () => {
  const self = useSelf();

  return (
    <aside class='components side root'>
      <A href='/'><img src={trophy} /></A>
      <A href={self() ? `/user'${self()!.id}` : window.location}><img src={user} /></A>
      <A href='/'>h</A>
    </aside >
  );
}
