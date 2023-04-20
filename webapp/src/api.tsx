import { invoke as r_invoke } from '@tauri-apps/api';

import wasm, * as W from '@/assets/artifacter_wasm';

const invoke = async (cmd: string, args: any) => {
  const r = await r_invoke(cmd, args);
  console.log(r);
  return r;
};

const is_tauri = window.__TAURI_METADATA__;

let first = true;

export async function prepare() {
  if (is_tauri) {
    return {
      get_profile: async (uid: number) => {
        return invoke('get_profile', { uid });
      },
      get_characters: async (uid: number, lang: W.Lang) => {
        return invoke('get_characters', {
          uid,
          lang,
        });
      },
      generate: async (
        uid: number,
        cid: number,
        lang: W.Lang,
        format: W.Format,
        counter: W.Counter
      ) => {
        return invoke('generate', {
          uid,
          cid,
          lang,
          format,
          counter,
        });
      },
    } as unknown as typeof W;
  } else {
    if (first) {
      await wasm();
      await W.w_load();
    }
    first = false;
    return W;
  }
}
