import React from 'react';
import { useRecoilState } from 'recoil';

import * as W from '@/assets/artifacter_wasm';
import { DownloadButton, GenerateButton } from '@/components/Button';
import { Divider } from '@/components/Divider';
import { DropdownMenu } from '@/components/DropdownMenu';
import { FormatType } from '@/components/FormatType';
import { useLocale } from '@/utils/locale';
import { formState } from '@/utils/recoil/formState';

export const SettingsPanel: React.FC = () => {
  const locale = useLocale();
  const [formStateValue, setFormState] = useRecoilState(formState);

  const langItems = [
    { label: '日本語', value: 'ja' },
    { label: 'English', value: 'en' },
  ];

  const counterItems = [
    { label: locale({ en: 'Normal', ja: '通常型' }), value: 'Normal' },
    { label: locale({ en: 'Hp', ja: 'HP型' }), value: 'Hp' },
    { label: locale({ en: 'Def', ja: '防御型' }), value: 'Def' },
    { label: locale({ en: 'Mastery', ja: '熟知型' }), value: 'ElementalMastery' },
    { label: locale({ en: 'Charge', ja: 'チャージ型' }), value: 'ChargeEfficiency' },
  ];

  return (
    <div className="flex flex-col h-full gap-5">
      <DropdownMenu
        items={langItems}
        value={formStateValue.lang}
        onChange={(value: string) =>
          setFormState({
            ...formStateValue,
            lang: value as W.Lang,
          })
        }
      />
      <DropdownMenu
        items={counterItems}
        value={formStateValue.counter}
        onChange={(value) => setFormState({ ...formStateValue, counter: value as W.Counter })}
      />
      <FormatType />
      <GenerateButton />
      <Divider />
      <DownloadButton />
    </div>
  );
};
