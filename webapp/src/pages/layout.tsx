import React from 'react';
import { Outlet } from 'react-router-dom';

import { Header } from '@/components/Header';

export const Layout: React.FC = () => {
  return (
    <div className="relative z-0 max-w-screen bg-primary min-h-screen h-full">
      <Header />
      <Outlet />
    </div>
  );
};
