git submodule sync
rm -rf build
cargo build -r --bin artifacter
mkdir -p build/assets
cp target/release/artifacter build/art
cp -r assets/trained build/assets/trained