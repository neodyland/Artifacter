rm dynamic-assets/loc.json
rm dynamic-assets/namecards.json
rm dynamic-assets/characters.json
curl.exe https://github.com/EnkaNetwork/API-docs/raw/master/store/loc.json -o dynamic-assets/loc.json -L
curl.exe https://github.com/EnkaNetwork/API-docs/raw/master/store/namecards.json -o dynamic-assets/namecards.json -L
curl.exe https://github.com/EnkaNetwork/API-docs/raw/master/store/characters.json -o dynamic-assets/characters.json -L