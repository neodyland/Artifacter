import { AnimatePresence } from 'framer-motion';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { RecoilRoot } from 'recoil';

// import App from './App';
import { IndexPage } from './pages';
import { GeneratePage } from './pages/generate';
import { Layout } from './pages/layout';
import { LocaleProvider } from './utils/locale';

import './index.css';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <LocaleProvider>
      <RecoilRoot>
        <BrowserRouter>
          <AnimatePresence mode="wait">
            <Routes>
              <Route path="/" element={<Layout />}>
                <Route index element={<IndexPage />} />
                <Route path="/generate" element={<GeneratePage />} />
              </Route>
            </Routes>
          </AnimatePresence>
        </BrowserRouter>
      </RecoilRoot>
    </LocaleProvider>
  </React.StrictMode>
);
