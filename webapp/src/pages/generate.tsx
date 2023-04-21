import React, { useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useRecoilState } from 'recoil';

import { prepare } from '@/api';
import { CharactersSelect } from '@/components/CharacterSelect';
import { ImageField } from '@/components/ImageField';
import { Loading } from '@/components/Loading';
import { SettingsPanel } from '@/components/SettingsPanel';
import { useLocaleState } from '@/utils/locale';
import { dataStore } from '@/utils/recoil/dataStore';
import { formState } from '@/utils/recoil/formState';

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
  const [data, setData] = useRecoilState(dataStore);
  const [formStateValue, setFormState] = useRecoilState(formState);

  useEffect(() => {
    if (isNaN(Number(uid)) || !uid || uid.length !== 9) {
      setIsLoading(true);
      navigate('/');
      setIsLoading(false);
    } else {
      (async () => {
        setIsLoading(true);
        const W = await prepare();
        const _lang = localeValue;
        const _characters = await W.get_characters(Number(uid), _lang);
        const _profile = await W.get_profile(Number(uid));
        setData({
          ...data,
          characters: _characters.map((c) => ({
            cid: c[0],
            name: c[1],
            level: c[2],
            elementName: c[3],
            imageDataUrl: c[4]
              ? `data:image/png;base64,${c[4]}`
              : 'https://upload-os-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_Hutao.png',
          })),
          profile: {
            nickname: _profile[0],
            signature: _profile[1],
            achievement: _profile[2],
            level: _profile[3],
            worldLevel: _profile[4],
            towerFloorIndex: _profile[5],
            towerLevelIndex: _profile[6],
            namecard: _profile[7],
          },
        });
        setFormState({
          ...formStateValue,
          uid: Number(uid),
          cid: _characters[0][0],
          lang: _lang,
        });
        setIsLoading(false);
      })();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [navigate, uid, localeValue]);

  return (
    <div className="mx-auto lg:h-screen min-h-screen max-w-screen-2xl px-4 md:px-8 lg:px-20 lg:pt-0">
      {isLoading ? (
        <Loading />
      ) : (
        <div className="text-white 2xl:pt-32 2xl:pb-32 pt-32 pb-8 h-full">
          <div className="lg:grid lg:grid-cols-7 lg:grid-rows-4 flex flex-col items-center h-full w-full gap-5 lg:px-0 px-4">
            <div className="lg:col-span-5 lg:row-span-1 order-1 w-full">
              <h1 className="font-primary text-6xl font-bold">{data.profile.nickname}</h1>
              <p className="font-primary text-sm text-gray-400 py-2">{data.profile.signature}</p>
            </div>
            <div className="lg:col-span-2 lg:row-span-1 order-2 w-full lg:order-2">
              <CharactersSelect characters={data.characters} />
            </div>
            <div className="lg:col-span-5 lg:row-span-3 lg:h-full w-full lg:pr-10 order-3">
              <ImageField />
            </div>
            <div className="lg:col-span-2 lg:row-span-3 h-full order-4 w-full">
              <SettingsPanel />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
