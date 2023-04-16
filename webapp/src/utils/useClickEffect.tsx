import { useEffect } from 'react';

export const useClickEffect = (ref: React.RefObject<HTMLDivElement>, callback: () => void) => {
  useEffect(() => {
    document.addEventListener('click', handleClick);
    return () => {
      document.removeEventListener('click', handleClick);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const handleClick = (e: any) => {
    if (ref.current && ref.current.contains(e.target)) {
      // Clicked inside dropdown, do nothing
      return;
    }
    // Clicked outside dropdown, close it
    callback();
  };
};
