import { motion } from 'framer-motion';
import React, { useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { useRecoilState, useRecoilValue } from 'recoil';

import * as W from '@/assets/artifacter_wasm';
import { Loading } from '@/components/Loading';
import { useLocale } from '@/utils/locale';
import { dataStore } from '@/utils/recoil/dataStore';
import { formState } from '@/utils/recoil/formState';

export const GenerateButton: React.FC = () => {
  const [data, setData] = useRecoilState(dataStore);
  const formStateValue = useRecoilValue(formState);

  const locale = useLocale();
  const [searchParams] = useSearchParams();
  const uid = searchParams.get('uid');

  const onClick = async () => {
    setData({ ...data, generateLoading: true });
    const { cid, lang, format, counter } = formStateValue;
    const imageData = await W.generate(Number(uid), cid, lang, format, counter);
    const imageDataUrl = `data:image/${format};base64,${imageData}`;
    setData({
      ...data,
      generatedImageDataUrl: imageDataUrl as `data:image/${string};base64,${string}`,
      generateLoading: false,
    });
  };

  return (
    <motion.button
      className={`text-white bg-secondary py-4 font-genshin text-sm rounded-md ring-white ring-1 ring-opacity-10 ${
        data.generateLoading
          ? 'h-[52px] cursor-not-allowed pointer-events-none'
          : 'hover:text-primary hover:bg-white'
      }`}
      onClick={() => onClick()}
      whileTap={{ scale: data.generateLoading ? 1 : 0.97 }}
      disabled={data.generateLoading}
    >
      {data.generateLoading ? <Loading size="sm" /> : locale({ en: 'Generate', ja: '生成する' })}
    </motion.button>
  );
};

export const DownloadButton = () => {
  const locale = useLocale();

  const [loading, setLoading] = useState(false);
  const { generatedImageDataUrl } = useRecoilValue(dataStore);

  const disabled = !generatedImageDataUrl;

  const download = (dataurl: `data:image/${string};base64,${string}`, filename: string) => {
    const link = document.createElement('a');
    link.href = dataurl;
    link.download = filename;
    link.click();
  };

  const onClick = () => {
    if (disabled) return;
    setLoading(true);
    download(
      generatedImageDataUrl,
      `artifacter.${generatedImageDataUrl.split(';')[0].split('/')[1]}`
    );
    setLoading(false);
  };

  return (
    <motion.button
      className={`text-white bg-secondary py-4 font-genshin text-sm rounded-md ring-white ring-1 ring-opacity-10 ${
        loading && 'h-[52px] cursor-not-allowed pointer-events-none'
      } ${
        disabled
          ? 'text-opacity-50 cursor-not-allowed pointer-events-none'
          : 'hover:text-primary hover:bg-white'
      }`}
      onClick={onClick}
      whileTap={{ scale: disabled ? 1 : 0.97 }}
      disabled={disabled}
    >
      {loading ? <Loading size="sm" /> : locale({ en: 'Download', ja: 'ダウンロード' })}
    </motion.button>
  );
};
