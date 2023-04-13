import { motion, useAnimation } from 'framer-motion';
import React, { useState } from 'react';
import { createSearchParams, useNavigate } from 'react-router-dom';

import top from '@/assets/top.jpg';
import { useLocale, useLocaleState } from '@/utils/locale';

const shakeVariants = {
  shake: {
    x: [-10, 10, -10, 10, 0],
    transition: { duration: 0.5 },
  },
};

const buttonVariants = {
  hover: {
    scale: 1.2,
    transition: { type: 'spring' },
  },
  tap: {
    scale: 0.9,
  },
  disabled: {
    scale: 1,
    transition: { type: 'spring' },
  },
};

export function IndexPage() {
  const locale = useLocale();
  const { localeValue } = useLocaleState();
  const shakeAnimationControls = useAnimation();
  const navigate = useNavigate();

  const [uid, setUid] = useState<string>('');

  const isDisabled = uid.length !== 9;

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { value } = event.target;

    if (isNaN(Number(value))) {
      shakeAnimationControls.start('shake');
      setUid(value.replace(/[^0-9]/g, ''));
    } else {
      setUid(value);
    }
  };

  const handleSubmit = (
    e: React.MouseEvent<HTMLButtonElement> | React.KeyboardEvent<HTMLInputElement>
  ) => {
    e.preventDefault();
    if (isDisabled) {
      shakeAnimationControls.start('shake');
    } else {
      navigate(
        `/generate?${createSearchParams({
          uid: uid,
        }).toString()}`
      );
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.nativeEvent.isComposing || e.key !== 'Enter') return;
    handleSubmit(e);
  };

  return (
    <div className="mx-auto h-full max-w-screen-2xl px-4 pt-32 md:px-8 lg:px-20 lg:pt-0 flex flex-col-reverse lg:flex-row">
      <div className="lg:flex-1">
        <div className="flex h-full flex-col justify-center">
          <h1 className="bg-clip-text my-6 font-primary font-bold text-6xl tracking-wider text-white md:text-7xl lg:py-2 lg:text-[5rem] 2xl:text-8xl">
            Generate Build Card
          </h1>
          <p
            className={`${
              localeValue === 'ja'
                ? 'font-notoSansJP text-lg font-light text-gray-100'
                : 'font-primary  text-xl'
            } py-2 tracking-wider text-white`}
          >
            {locale({
              en: 'Unofficial app for Genshin Impact.',
              ja: '原神のためのツール',
            })}
            <br />
            {locale({
              en: "Let's generate build cards!",
              ja: 'ビルドカードを生成しましょう！',
            })}
          </p>
          <div className="pt-6">
            <motion.div
              className="relative block w-3/4"
              onChange={handleChange}
              variants={shakeVariants}
              animate={shakeAnimationControls}
            >
              <input
                className="font-genshin bg-white bg-opacity-5 backdrop-blur-md py-5 px-4 w-full rounded-lg text-white outline-none text-xl"
                placeholder="Enter your uid"
                type="text"
                name="url"
                value={uid}
                onKeyDown={handleKeyDown}
              />
              <motion.button
                type="submit"
                className="absolute right-0 inset-y-0 px-4 py-2"
                onClick={handleSubmit}
                whileHover={isDisabled ? 'disabled' : 'hover'}
                whileTap={isDisabled ? 'disabled' : 'tap'}
                variants={buttonVariants}
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                  strokeWidth={1.5}
                  stroke="currentColor"
                  className={`w-6 h-6 ${isDisabled ? 'text-gray-500' : 'text-white'}`}
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75"
                  />
                </svg>
              </motion.button>
            </motion.div>
          </div>
        </div>
      </div>
      <div className="flex-1">
        <div className="absolute top-0 left-[-50%] -z-10 w-[200%] lg:left-auto lg:right-0 lg:w-[80%] xl:w-[85%] 2xl:w-[75%] ">
          <div className="relative h-[calc(100%+20px)] w-full before:absolute before:inset-y-[-10px] before:inset-x-0 before:z-10 before:bg-video before:content-[''] lg:w-[calc(100%+2px)] lg:before:inset-x-[-1px]">
            <img
              src={top}
              className="z-0 mt-[-10px] mr-[-8px] hidden w-full lg:block"
              alt="top"
            ></img>
          </div>
        </div>
      </div>
    </div>
  );
}
