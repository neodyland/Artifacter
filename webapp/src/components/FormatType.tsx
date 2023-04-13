import React from 'react';
import { useRecoilState } from 'recoil';

import { formState } from '@/utils/recoil/formState';

export const FormatType = () => {
  const [formStateValue, setFormState] = useRecoilState(formState);
  return (
    <div className="w-full flex gap-4 font-genshin pb-3 text-sm">
      <button
        className={`py-4 w-full ring-white ring-1 ring-opacity-10 transition-colors duration-300 ease-in-out ${
          formStateValue.format === 'png' ? 'bg-white text-primary' : 'bg-secondary text-white'
        } rounded-md`}
        onClick={() => setFormState({ ...formStateValue, format: 'png' })}
      >
        PNG
      </button>
      <button
        className={`py-4 w-full ring-white ring-1 ring-opacity-10 transition-colors duration-300 ease-in-out ${
          formStateValue.format === 'jpeg' ? 'bg-white text-primary' : 'bg-secondary text-white'
        } rounded-md`}
        onClick={() => setFormState({ ...formStateValue, format: 'jpeg' })}
      >
        JPEG
      </button>
    </div>
  );
};
