import { motion, AnimatePresence } from 'framer-motion';
import React, { useState } from 'react';
import { useRecoilState } from 'recoil';

import * as W from '../assets/artifacter_wasm';

import { CharacterCard } from './CharacterCard';

import { formState } from '@/utils/recoil/formState';
import { useClickEffect } from '@/utils/useClickEffect';

type Props = {
  characters: W.Character[];
};

export const CharactersSelect: React.FC<Props> = ({ characters }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [formStateValue, setFormState] = useRecoilState(formState);

  const dropdownRef = React.useRef<HTMLDivElement>(null);

  useClickEffect(dropdownRef, () => setIsOpen(false));

  const toggleOpen = () => setIsOpen(!isOpen);

  const selectedCharacter = characters.find((c) => c.cid === formStateValue.cid) as W.Character;

  return (
    <div className="relative w-[calc(100%+2px)]" ref={dropdownRef}>
      <CharacterCard character={selectedCharacter} onClick={toggleOpen} />
      <AnimatePresence>
        {isOpen && (
          <motion.ul
            className="absolute z-10 mt-3 py-1 w-full inset-x-[-1px] rounded-sm shadow-lg bg-secondary ring-1 ring-white ring-opacity-10 focus:outline-none"
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
          >
            {characters.map((c) => (
              <motion.li
                key={c.cid}
                className="block px-4 py-2 text-sm hover:bg-white text-white hover:text-gray-900"
                whileHover={{ scale: 1.03 }}
                onClick={() => {
                  setFormState({ ...formStateValue, cid: c.cid });
                  setIsOpen(false);
                }}
                exit={{ opacity: 0 }}
              >
                <div className="flex flex-col items-start justify-center font-genshin">
                  <h3 className="text-base">{c.name}</h3>
                  <p className="text-xs">Lv.{c.level}</p>
                </div>
              </motion.li>
            ))}
          </motion.ul>
        )}
      </AnimatePresence>
    </div>
  );
};
