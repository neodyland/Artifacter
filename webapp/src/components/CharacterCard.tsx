import { motion, useAnimation } from 'framer-motion';
import React, { useEffect } from 'react';
import { API } from '@/api';
type ColorNames = 'pyro' | 'electro' | 'hydro' | 'dendro' | 'anemo' | 'geo' | 'cryo';

type Props = {
  character: API.Character;
  onClick: () => void;
};

export const CharacterCard: React.FC<Props> = ({ character, onClick }) => {
  const opacityAnimationControls = useAnimation();

  const bgColorVariants = {
    pyro: 'bg-pyro',
    electro: 'bg-electro',
    hydro: 'bg-hydro',
    dendro: 'bg-dendro',
    anemo: 'bg-anemo',
    geo: 'bg-geo',
    cryo: 'bg-cryo',
  };

  const toColorVariants = {
    pyro: 'to-pyro',
    electro: 'to-electro',
    hydro: 'to-hydro',
    dendro: 'to-dendro',
    anemo: 'to-anemo',
    geo: 'to-geo',
    cryo: 'to-cryo',
  };

  const elementColors = (elementName: string) => {
    switch (elementName.charAt(0)) {
      case '炎':
        return 'pyro';
      case '雷':
        return 'electro';
      case '水':
        return 'hydro';
      case '草':
        return 'dendro';
      case '風':
        return 'anemo';
      case '岩':
        return 'geo';
      case '氷':
        return 'cryo';
      default:
        switch (elementName) {
          case 'FIRE':
            return 'pyro';
          case 'WATER':
            return 'hydro';
          case 'WIND':
            return 'anemo';
          case 'ELEC':
            return 'electro';
          case 'GRASS':
            return 'dendro';
          case 'ICE':
            return 'cryo';
          case 'ROCK':
            return 'geo';
          default:
            console.error('Unknown element name: ' + elementName);
        }
    }
  };

  useEffect(() => {
    opacityAnimationControls.start({ opacity: 0, transition: { duration: 0 } });
    setTimeout(() => {
      opacityAnimationControls.start({
        opacity: 1,
        transition: { duration: 0.25, ease: 'easeInOut' },
      });
    }, 250);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [character]);

  const colorName = elementColors(character.element) as ColorNames;

  return (
    <button type="button" className="w-[calc(100%-2px)]" onClick={onClick}>
      <div
        className={`flex items-center ${bgColorVariants[colorName]} rounded-lg px-4 w-full h-24 transition-all duration-500 ease-in-out`}
      >
        <motion.div className="relative h-full w-auto" animate={opacityAnimationControls}>
          <img src={character.imageURL} alt="your_image" className="z-0 h-full w-auto" />
          <div
            className={`absolute inset-0 bg-gradient-to-r from-transparent ${toColorVariants[colorName]} z-10 h-full w-full`}
          />
        </motion.div>
        <motion.div
          className="flex flex-col gap-1 text-white font-genshin p-4 items-start"
          animate={opacityAnimationControls}
        >
          <h3 className="text-xl">{character.name}</h3>
          <p className="text-sm">Lv.{character.level}</p>
        </motion.div>
      </div>
    </button>
  );
};
