rm -rf build
cargo build -r
mkdir -p build
mv target/release/artifacter build/art
cp -r assets build/assets
rm build/assets/subop.json
rm build/assets/dupe.json
rm build/assets/font.ttf