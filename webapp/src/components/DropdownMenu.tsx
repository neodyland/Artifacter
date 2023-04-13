import { motion, AnimatePresence } from 'framer-motion';
import React, { useState } from 'react';

import { useClickEffect } from '@/utils/useClickEffect';

type Props = {
  items: {
    label: string;
    value: string;
  }[];
  value: string;
  onChange: (value: string) => void;
};

export const DropdownMenu: React.FC<Props> = ({ items, value, onChange }) => {
  const [isOpen, setIsOpen] = useState(false);
  const [itemList] = useState(items);
  const toggleOpen = () => setIsOpen(!isOpen);

  const dropdownRef = React.useRef<HTMLDivElement>(null);

  const selectedItem = items.find((i) => i.value === value);

  useClickEffect(dropdownRef, () => setIsOpen(false));

  return (
    <div className="relative w-[calc(100%+2px)]" ref={dropdownRef}>
      <button
        className="w-[calc(100%-2px)] flex justify-between items-center px-4 py-5 text-sm font-medium font-genshin text-white bg-secondary rounded-sm outline-none ring-1 ring-white ring-opacity-10"
        onClick={toggleOpen}
      >
        <span>{selectedItem?.label}</span>
        <svg
          className="w-5 h-5"
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
          aria-hidden="true"
        >
          <path
            fillRule="evenodd"
            d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
            clipRule="evenodd"
          />
        </svg>
      </button>
      <AnimatePresence>
        {isOpen && (
          <motion.ul
            className="absolute z-10 mt-3 py-2 inset-x-[-1px] rounded-md shadow-lg bg-secondary ring-1 ring-white ring-opacity-10 focus:outline-none"
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
          >
            {itemList.map((item, _) => (
              <motion.li
                key={_}
                className="block px-4 py-2 text-sm hover:bg-white bg-opacity-5 text-white hover:text-primary"
                whileHover={{ scale: 1.03 }}
                onClick={() => {
                  onChange(item.value);
                  setIsOpen(false);
                }}
              >
                <div className="flex flex-col items-start justify-center font-genshin">
                  <h3 className="text-sm">{item.label}</h3>
                </div>
              </motion.li>
            ))}
          </motion.ul>
        )}
      </AnimatePresence>
    </div>
  );
};
