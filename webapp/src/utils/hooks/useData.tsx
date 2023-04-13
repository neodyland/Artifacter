import { useState } from 'react';

import * as W from '../../assets/artifacter_wasm';

export type DataStateType = {
  characters: W.Character[];
  profile: W.UserProfile;
  generatedImageDataUrl?: `data:image/${string};base64,${string}`;
};

export const useData = () => {
  const [data, setData] = useState<DataStateType>({
    characters: [],
    profile: {
      nickname: '',
      signature: '',
      achievement: 0,
      level: 0,
      worldLevel: 0,
      towerFloorIndex: 0,
      towerLevelIndex: 0,
      namecard: '',
    },
  });

  const setDataFromArray = (characters: W.ArrayCharacter[], profile: W.ArrayProfile) => {
    setData({
      ...data,
      characters: characters.map((c) => ({
        cid: c[0],
        name: c[1],
        level: c[2],
        elementName: c[3],
        imageDataUrl: c[4]
          ? `data:image/png;base64,${c[4]}`
          : 'https://upload-os-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_Hutao.png',
      })),
      profile: {
        nickname: profile[0],
        signature: profile[1],
        achievement: profile[2],
        level: profile[3],
        worldLevel: profile[4],
        towerFloorIndex: profile[5],
        towerLevelIndex: profile[6],
        namecard: profile[7],
      },
    });
  };

  return { data, setData, setDataFromArray };
};
