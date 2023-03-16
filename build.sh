rm -rf build
cargo build -r --bin artifacter
mkdir -p build
cp target/release/artifacter build/art