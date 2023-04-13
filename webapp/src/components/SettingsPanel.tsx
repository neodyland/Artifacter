import React from 'react';

import { Button } from './Button';
import { Divider } from './Divider';
import { DropdownMenu } from './DropdownMenu';
import { FormatType } from './FormatType';

import { FormState } from '@/pages/generate';
import { useLocale } from '@/utils/locale';

type Props = {
  formState: FormState;
  setFormState: React.Dispatch<React.SetStateAction<FormState>>;
  generate: (cid: number, lang: string, format: string, counter: string) => Promise<boolean>;
  generateLoading: boolean;
};

export const SettingsPanel: React.FC<Props> = ({
  formState,
  setFormState,
  generate,
  generateLoading,
}) => {
  const locale = useLocale();
  const langItems = [
    { label: '日本語', value: 'Ja' },
    { label: 'English', value: 'En' },
  ];

  const counterItems = [
    { label: locale({ en: 'Normal', ja: '通常型' }), value: 'Normal' },
    { label: locale({ en: 'Hp', ja: 'HP型' }), value: 'Hp' },
    { label: locale({ en: 'Def', ja: '防御型' }), value: 'Def' },
    { label: locale({ en: 'Mastery', ja: '熟知型' }), value: 'Mastery' },
    { label: locale({ en: 'Charge', ja: 'チャージ型' }), value: 'Charge' },
  ];

  const handleGenerate = async () => {
    const { cid, lang, format, counter } = formState;
    await generate(cid, lang, format, counter);
    console.log('a');
  };

  return (
    <div className="flex flex-col h-full gap-5">
      <DropdownMenu
        items={langItems}
        value={formState.lang}
        onChange={(value) =>
          setFormState({
            ...formState,
            lang: value,
          })
        }
      />
      <DropdownMenu
        items={counterItems}
        value={formState.counter}
        onChange={(value) => setFormState({ ...formState, counter: value })}
      />
      <FormatType formState={formState} setFormState={setFormState} />
      <Button onClick={handleGenerate} loading={generateLoading}>
        {locale({ en: 'Generate', ja: '生成する' })}
      </Button>
      <Divider />
      <Button disabled onClick={() => console.log('clicked')}>
        {locale({ en: 'Download', ja: 'ダウンロード' })}
      </Button>
    </div>
  );
};
