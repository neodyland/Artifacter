/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: '#040A27',
        secondary: '#101631',
        discord: '#5865f2',
        pyro: '#d2655a',
        electro: '#b681df',
        hydro: '#559cc9',
        dendro: '#3AAF7A',
        anemo: '#5ebd74',
        geo: '#BF8F37',
        cryo: '#559cc9',
        gradient: {
          1: '#2F80ED',
          2: '#6F42C1',
        },
      },
      fontFamily: {
        genshin: ['"Genshin"'],
        primary: ['"Gilroy"', '"Noto Sans JP"', 'sans-serif'],
        notoSansJP: ['"Noto Sans JP"', 'sans-serif'],
      },
      backgroundImage: {
        video:
          'linear-gradient(to bottom, transparent 0 60%, #040A27 100%), linear-gradient(to left, transparent 0 60%, #040A27 100%), radial-gradient(at right top, transparent 60%, #040A27 100%), radial-gradient(at left top, transparent 60%, #040A27 100%), radial-gradient(at left bottom, transparent 60%, #040A27 100%), radial-gradient(at right bottom, transparent 60%, #040A27 100%)',
      },
    },
  },
  plugins: [],
};
