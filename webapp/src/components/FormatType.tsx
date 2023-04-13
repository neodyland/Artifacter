import React from 'react';

import { FormState } from '@/pages/generate';

type Props = {
  formState: FormState;
  setFormState: React.Dispatch<React.SetStateAction<FormState>>;
};

export const FormatType: React.FC<Props> = ({ formState, setFormState }) => {
  return (
    <div className="w-full flex gap-4 font-genshin pb-3 text-sm">
      <button
        className={`py-4 w-full ring-white ring-1 ring-opacity-10 transition-colors duration-300 ease-in-out ${
          formState.format === 'png' ? 'bg-white text-primary' : 'bg-secondary text-white'
        } rounded-md`}
        onClick={() => setFormState({ ...formState, format: 'png' })}
      >
        PNG
      </button>
      <button
        className={`py-4 w-full ring-white ring-1 ring-opacity-10 transition-colors duration-300 ease-in-out ${
          formState.format === 'jpeg' ? 'bg-white text-primary' : 'bg-secondary text-white'
        } rounded-md`}
        onClick={() => setFormState({ ...formState, format: 'jpeg' })}
      >
        JPEG
      </button>
    </div>
  );
};
