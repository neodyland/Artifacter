cargo build -r
mkdir -p build
mv target/release/artifacter build/art
cp -r assets build/assets