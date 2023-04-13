import artifacterLogo from '@/assets/artifacter-logo.svg';
import { useLocaleState } from '@/utils/locale';

export function Header() {
  const { localeValue, setLocaleState } = useLocaleState();

  return (
    <header className="fixed inset-x-0 top-0 z-10 flex items-center justify-between px-4 text-white transition-[padding-top,padding-bottom] md:px-8 lg:px-20 2xl:px-[calc((100vw-1536px)/2+5rem)] py-4 md:py-8">
      <div className="flex h-full gap-12">
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

        <nav className="hidden gap-12 lg:flex"></nav>
      </div>
      <div className="-ml-8 hidden flex-col gap-2.5 sm:flex-row sm:justify-center lg:flex lg:justify-start">
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
