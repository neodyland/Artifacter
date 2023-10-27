import { atom } from 'recoil';
import { API } from '@/api';

export type FormState = {
  uid: number;
  cid: number;
  lang: API.Lang;
  format: API.Format;
  counter: API.Counter;
};

export const formState = atom<FormState>({
  key: 'formState',
  default: {
    uid: 0,
    cid: 0,
    lang: 'ja',
    format: 'png',
    counter: 'Normal',
  },
});
