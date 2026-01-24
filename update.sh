rm dynamic-assets/loc.json
rm dynamic-assets/namecards.json
rm dynamic-assets/characters.json
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/gi/locs.json -o dynamic-assets/loc1.json -L
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/loc.json -o dynamic-assets/loc2.json -L
jq -s '.[0] * .[1]' dynamic-assets/loc1.json dynamic-assets/loc2.json > dynamic-assets/loc.json
rm dynamic-assets/loc1.json
rm dynamic-assets/loc2.json
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/gi/namecards.json -o dynamic-assets/namecards.json -L
sed -i 's/\/ui\///g' dynamic-assets/namecards.json
sed -i 's/.jpg//g' dynamic-assets/namecards.json
sed -i 's/Icon/icon/g' dynamic-assets/namecards.json
curl https://github.com/EnkaNetwork/API-docs/raw/master/store/gi/avatars.json -o dynamic-assets/characters.json -L
sed -i 's/\/ui\///g' dynamic-assets/characters.json
sed -i 's/.png//g' dynamic-assets/characters.json