import { atom } from 'recoil';
import { API } from '@/api';

export type DataStoreType = {
  characters: API.Character[];
  profile: API.Profile;
  generateLoading: boolean;
  generatedImageDataUrl: `data:image/${string};base64,${string}` | null;
};

export const dataStore = atom<DataStoreType>({
  key: 'dataStore',
  default: {
    characters: [],
    profile: {
      name: '',
      description: '',
      achievement: 0,
      level: 0,
      worldLevel: 0,
      towerFloorIndex: 0,
      towerLevelIndex: 0,
      namecard: '',
    },
    generateLoading: false,
    generatedImageDataUrl: null,
  },
});
