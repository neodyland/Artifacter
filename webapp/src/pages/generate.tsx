import React, { useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';

import wasm, * as W from '../assets/artifacter_wasm';

import { CharactersSelect } from '@/components/CharacterSelect';
import { ImageField } from '@/components/ImageField';
import { Loading } from '@/components/Loading';
import { SettingsPanel } from '@/components/SettingsPanel';
import { useData } from '@/utils/hooks/useData';
import { useGenerate } from '@/utils/hooks/useGenerate';
import { useLocaleState } from '@/utils/locale';

export type FormState = {
  uid: number;
  cid: number;
  lang: string;
  format: string;
  counter: string;
};

export const GeneratePage: React.FC = () => {
  const navigate = useNavigate();
  const { localeValue } = useLocaleState();

  const [searchParams] = useSearchParams();
  const uid = searchParams.get('uid');

  const [isLoading, setIsLoading] = useState(true);
  const [formState, setFormState] = useState<FormState>({
    uid: 0,
    cid: 0,
    lang: '',
    format: 'png',
    counter: 'Normal',
  });

  const { data, setData, setDataFromArray } = useData();
  const { isLoading: generateLoading, generate } = useGenerate(data, setData);

  useEffect(() => {
    console.log(data);
  }, [data]);

  useEffect(() => {
    if (isNaN(Number(uid)) || !uid || uid.length !== 9) {
      setIsLoading(true);
      navigate('/');
      setIsLoading(false);
    } else {
      (async () => {
        setIsLoading(true);
        await wasm();
        const { w_load } = W;
        await w_load();
        const lang = localeValue === 'ja' ? 'Ja' : 'En';
        const _characters = await W.get_characters(Number(uid), lang);
        console.log(lang, _characters);
        const _profile = await W.get_profile(Number(uid));
        setDataFromArray(_characters, _profile);
        setFormState({
          ...formState,
          uid: Number(uid),
          cid: _characters[0][0],
          lang: lang,
        });
        setIsLoading(false);
      })();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [navigate, uid, localeValue]);

  return (
    <div className="mx-auto h-full max-w-screen-2xl px-4 md:px-8 lg:px-20 lg:pt-0">
      {isLoading ? (
        <Loading />
      ) : (
        <div className="text-white 2xl:pt-32 2xl:pb-32 pt-32 pb-8 h-full">
          <div className="grid grid-cols-7 grid-rows-4 items-center h-full">
            <div className="col-span-5 row-span-1">
              <h1 className="font-primary text-6xl font-bold">{data.profile.nickname}</h1>
              <p className="font-primary text-sm text-gray-400 py-2">{data.profile.signature}</p>
            </div>
            <div className="col-span-2 row-span-1">
              <CharactersSelect
                characters={data.characters}
                formState={formState}
                setFormState={setFormState}
              />
            </div>
            <div className="col-span-5 row-span-3 h-full w-full pr-10">
              <ImageField imageDataUrl={data.generatedImageDataUrl} loading={generateLoading} />
            </div>
            <div className="col-span-2 row-span-3 h-full">
              <SettingsPanel
                generate={generate}
                generateLoading={generateLoading}
                formState={formState}
                setFormState={setFormState}
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
