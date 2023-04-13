import React from 'react';

type Props = {
  children: React.ReactNode;
};

export const Center: React.FC<Props> = ({ children }) => {
  return <div className="flex items-center justify-center">{children}</div>;
};
