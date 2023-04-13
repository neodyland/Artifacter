import { atom } from "recoil";

import * as W from "../../assets/artifacter_wasm";

export type FormState = {
    uid: number;
    cid: number;
    lang: W.Lang;
    format: W.Format;
    counter: W.Counter;
};

export const formState = atom<FormState>({
    key: 'formState',
    default: {
        uid: 0,
        cid: 0,
        lang: 'Ja',
        format: 'png',
        counter: 'Normal',
    },
});