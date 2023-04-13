import React, { useState } from 'react';
import { useSearchParams } from 'react-router-dom';

import * as W from '../../assets/artifacter_wasm';

import { DataStateType } from './useData';

export const useGenerate = (
  data: DataStateType,
  setData: React.Dispatch<React.SetStateAction<DataStateType>>
) => {
  const [isLoading, setIsLoading] = useState(false);

  const [searchParams] = useSearchParams();
  const uid = searchParams.get('uid');

  const generate = async (cid: number, lang: string, format: string, counter: string) => {
    try {
      setIsLoading(true);
      const _image = await W.generate(Number(uid), cid, lang, format, counter);
      setData({
        ...data,
        generatedImageDataUrl: `data:image/${format};base64,${_image}`,
      });
      console.log(data);
      setIsLoading(false);
      return true;
    } catch (e) {
      console.error(e);
      setIsLoading(false);
      return false;
    }
  };

  return { isLoading, generate };
};
