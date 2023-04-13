import React from 'react';
import { Outlet } from 'react-router-dom';

import { Header } from '@/components/Header';

export const Layout: React.FC = () => {
  return (
    <div className="relative z-0 max-w-[100vw] bg-primary h-screen">
      <Header />
      <Outlet />
    </div>
  );
};
