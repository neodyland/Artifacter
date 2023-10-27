export namespace API {
  export interface Character {
    name: string;
    id: number;
    level: number;
    imageURL: string;
    element: string;
  }
  export interface Profile {
    name: string;
    description: string;
    level: number;
    achievement: number;
    worldLevel: number;
    towerFloorIndex: number;
    towerLevelIndex: number;
    namecard?: string;
  }
  export type Lang = 'ja' | 'en';
  export type Format = 'png' | 'jpeg' | 'raw';
  export type Counter = 'Normal' | 'Hp' | 'Def' | 'ElementalMastery' | 'ChargeEfficiency';
  export interface RawCharacter {
    element_name: string;
    name: string;
    id: number;
    level: number;
    icon: string;
  }
  export interface RawProfile {
    name: string;
    description: string;
    achievement: number;
    world_level: number;
    floor: number;
    level: number;
    name_card?: string;
    characters: RawCharacter[];
  }
}
const HOSTNAME = import.meta.env.DEV ? 'http://localhost:3000' : 'https://ag-api.neody.land';

export async function generate(
  uid: number,
  cid: number,
  lang: API.Lang,
  format: API.Format,
  counter: API.Counter
): Promise<string> {
  const response = await fetch(
    `${HOSTNAME}/generate?uid=${uid}&cid=${cid}&lang=${lang}&format=${format}&counter=${counter}`
  );
  const data = await response.blob();
  const url = URL.createObjectURL(data);
  return url;
}

export async function getProfile(uid: number, lang: API.Lang): Promise<API.RawProfile> {
  const response = await fetch(`${HOSTNAME}/profile?uid=${uid}&lang=${lang}`);
  const data = await response.json();
  return data;
}
