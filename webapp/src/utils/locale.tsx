/*eslint linebreak-style: ["error", "windows"]*/

import { createContext, FunctionComponent, useContext, useEffect, useState } from 'react';

export const Locales = {
  en: {
    name: 'ENG',
    countryCode: 'gb',
  },
  ja: {
    name: 'JPN',
    countryCode: 'jp',
  },
} as const;

export type Locale = keyof typeof Locales;

export type LocaleObject<T> = { en: T } & {
  [key in Locale]?: T;
};

const LocaleContext = createContext<{
  locale: Locale;
  setLocale: (locale: Locale) => void;
}>({ locale: 'en', setLocale: () => null });

export interface LocaleProviderContext {
  children: JSX.Element;
}

export const LocaleProvider: FunctionComponent<LocaleProviderContext> = (props) => {
  const [locale, setLocale] = useState<Locale>('en');
  useEffect(function () {
    setLocale(
      (localStorage.getItem('locale') as Locale) ??
        (window.navigator.language === 'ja-JP' ? 'ja' : 'en')
    );
  }, []);
  return (
    <LocaleContext.Provider
      value={{
        locale,
        setLocale: (locale: Locale) => {
          setLocale(locale);
          localStorage.setItem('locale', locale);
        },
      }}
    >
      {props.children}
    </LocaleContext.Provider>
  );
};

export const useLocaleState = () => {
  const { locale: localeValue, setLocale: setLocaleState } = useContext(LocaleContext);
  return { localeValue, setLocaleState };
};

export const useLocale = () => {
  const { locale } = useContext(LocaleContext);
  return <T = JSX.Element,>(locales: LocaleObject<T>) => {
    return locales[locale] || locales.en;
  };
};
