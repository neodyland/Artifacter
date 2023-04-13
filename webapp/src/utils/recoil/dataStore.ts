import { atom } from 'recoil';

import * as W from '../../assets/artifacter_wasm'

export type DataStoreType = {
    characters: W.Character[];
    profile: W.UserProfile;
    generateLoading: boolean;
    generatedImageDataUrl: `data:image/${string};base64,${string}` | null;
};

export const dataStore = atom<DataStoreType>({
    key: 'dataStore',
    default: {
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
        generateLoading: false,
        generatedImageDataUrl: null,
    },
});