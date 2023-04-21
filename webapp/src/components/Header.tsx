import { useMotionValueEvent, useScroll } from 'framer-motion';
import { useState } from 'react';

import artifacterLogo from '@/assets/artifacter-logo.svg';
import { useLocaleState } from '@/utils/locale';

export function Header() {
  const { localeValue, setLocaleState } = useLocaleState();

  const { scrollY } = useScroll();
  const [scrolled, setScrolled] = useState(false);

  useMotionValueEvent(scrollY, 'change', (latest) => {
    latest > 10 ? setScrolled(true) : setScrolled(false);
  });

  return (
    <header
      className={`flex justify-between fixed inset-x-0 top-0 z-10 w-full items-center px-4 text-white transition-[padding-top,padding-bottom] md:px-8 lg:px-20 2xl:px-[calc((100vw-1536px)/2+5rem)] ${
        scrolled ? 'py-4 backdrop-blur-md bg-primary bg-opacity-70' : 'py-8'
      }`}
    >
      <div className="flex h-full gap-12 w-full justify-between lg:justify-normal">
        <div>
          <a href="/" aria-label="logo">
            <img src={artifacterLogo} alt="logo" className="h-10 w-auto md:h-11" />
          </a>
        </div>

        <button
          onClick={() => setLocaleState(localeValue === 'en' ? 'ja' : 'en')}
          className="text-md ml-8 mr-4 w-16 rounded-lg px-5 text-center font-genshin tracking-wider text-white outline outline-1"
        >
          {localeValue.charAt(0).toUpperCase() + localeValue.slice(1)}
        </button>
      </div>
      <div className="-ml-8 w-full hidden lg:flex lg:justify-end">
        <a
          href="https://artifacter.neody.land"
          target="_blank"
          className="text-md inline-block rounded-lg px-5 py-3 text-center font-genshin text-white outline outline-1 transition duration-100"
          rel="noreferrer"
        >
          Discord Bot
        </a>
      </div>
    </header>
  );
}
