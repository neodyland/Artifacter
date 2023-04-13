import { motion } from 'framer-motion';
import React from 'react';

import { Loading } from './Loading';

type Props = {
  onClick?: () => void;
  disabled?: boolean;
  loading?: boolean;
  children: React.ReactNode;
};

export const Button: React.FC<Props> = ({
  onClick,
  disabled = false,
  loading = false,
  children,
}) => {
  return (
    <motion.button
      className={`text-white bg-secondary py-4 font-genshin text-sm rounded-md ring-white ring-1 ring-opacity-10 ${
        loading && 'h-[52px] cursor-not-allowed pointer-events-none'
      } ${
        disabled
          ? 'text-opacity-50 cursor-not-allowed pointer-events-none'
          : 'hover:text-primary hover:bg-white'
      }`}
      onClick={onClick}
      whileTap={{ scale: disabled ? 1 : 0.97 }}
      disabled={disabled}
    >
      {loading ? <Loading size="sm" /> : children}
    </motion.button>
  );
};
