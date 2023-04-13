/* eslint-disable jsx-a11y/alt-text */

import { useEffect, useState } from 'react';

import wasm, * as W from './assets/artifacter_wasm';

function App() {
  useEffect(() => {
    (async () => {
      await wasm();
      const { w_load } = W;
      await w_load();
    })();
  }, []);

  const [formState, setFormState] = useState({
    uid: '827106332',
    cid: '1',
    lang: 'Ja',
    format: 'png',
    counter: 'Normal',
  });
  const [characters, setCharacters] = useState('');
  const [image, setImage] = useState<string | null>(null);

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = event.target;
    setFormState({
      ...formState,
      [name]: value,
    });
  };

  const getCharacters = async () => {
    const lang = 'Ja';
    const chars = await W.get_characters(Number(formState.uid), lang);
    setCharacters(chars.toString());
  };

  const generate = async () => {
    const { uid, cid, lang, format, counter } = formState;
    const image = await W.generate(Number(uid), Number(cid), lang, format, counter);
    setImage(`data:image/png;base64,${image}`);
  };

  return (
    <div>
      <h1 className="text-red-500">AAAAAAA</h1>
      <input
        name="uid"
        value={formState.uid}
        onChange={(e) => handleInputChange(e)}
        placeholder="Here is a sample placeholder"
      />
      <button onClick={() => getCharacters()}>Get characters</button>
      <p>{characters}</p>
      <input
        name="cid"
        value={formState.cid}
        onChange={(e) => handleInputChange(e)}
        placeholder="Here is a sample placeholder"
      />
      <input
        name="lang"
        value={formState.lang}
        onChange={(e) => handleInputChange(e)}
        placeholder="Here is a sample placeholder"
      />
      <input
        name="format"
        value={formState.format}
        onChange={(e) => handleInputChange(e)}
        placeholder="Here is a sample placeholder"
      />
      <input
        name="counter"
        value={formState.counter}
        onChange={(e) => handleInputChange(e)}
        placeholder="Here is a sample placeholder"
      />
      <button onClick={() => generate()}>Generate</button>
      {image && <img src={image} />}
    </div>
  );
}

export default App;
