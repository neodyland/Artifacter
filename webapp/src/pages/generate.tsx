import React, { useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { useRecoilState } from 'recoil';

import { getProfile } from '@/api';
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
        const _lang = localeValue;
        const profile = await getProfile(Number(uid), _lang);
        setData({
          ...data,
          characters: profile.characters.map((c) => ({
            id: c.id,
            name: c.name,
            level: c.level,
            element: c.element_name,
            imageURL: `data:image/png;base64,${c.icon}`,
          })),
          profile: {
            name: profile.name,
            description: profile.description,
            achievement: profile.achievement,
            level: profile.level,
            worldLevel: profile.world_level,
            towerFloorIndex: profile.floor,
            towerLevelIndex: profile.level,
            namecard: profile.name_card,
          },
        });
        setFormState({
          ...formStateValue,
          uid: Number(uid),
          cid: profile.characters[0].id,
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
              <h1 className="font-primary text-6xl font-bold">{data.profile.name}</h1>
              <p className="font-primary text-sm text-gray-400 py-2">{data.profile.description}</p>
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
