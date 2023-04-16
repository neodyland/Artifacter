import { motion } from 'framer-motion';
import React from 'react';

type Props = {
  size?: 'sm' | 'md' | 'lg';
};
export const Loading: React.FC<Props> = ({ size = 'lg' }) => {
  const sizeClassVariants = {
    sm: 'w-2 h-2 mr-0',
    md: 'w-3 h-3 mr-1',
    lg: 'w-4 h-4 mr-2',
  };

  const animateVariants = {
    sm: { y: [0, -4, 0], opacity: [1, 0.7, 1] },
    md: { y: [0, -6, 0], opacity: [1, 0.7, 1] },
    lg: { y: [0, -8, 0], opacity: [1, 0.7, 1] },
  };

  const sizeClass = sizeClassVariants[size];
  const animate = animateVariants[size];

  return (
    <div className="flex justify-center items-center w-full h-full gap-1">
      <motion.div
        className={`${sizeClass} bg-white rounded-full`}
        animate={animate}
        transition={{ duration: 0.8, repeat: Infinity, ease: 'easeInOut', repeatDelay: 0.4 }}
      ></motion.div>
      <motion.div
        className={`${sizeClass} bg-white rounded-full`}
        animate={animate}
        transition={{
          duration: 0.8,
          repeat: Infinity,
          delay: 0.2,
          ease: 'easeInOut',
          repeatDelay: 0.4,
        }}
      ></motion.div>
      <motion.div
        className={`${sizeClass} bg-white rounded-full`}
        animate={animate}
        transition={{
          duration: 0.8,
          repeat: Infinity,
          delay: 0.4,
          ease: 'easeInOut',
          repeatDelay: 0.4,
        }}
      ></motion.div>
    </div>
  );
};
