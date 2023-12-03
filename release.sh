#!/usr/bin/bash

version_line=$(grep "^version" Cargo.toml)
readarray -d "\"" -t version <<< "$version_line"

echo "Building Linux"
cargo build --release
echo "Building Windows"
RUSTFLAGS='-L /usr/x86_64-w64-mingw32/lib' cargo zigbuild --target x86_64-pc-windows-gnu --release

cd target/release
zip -r "../linux-${version[1]}.zip" honeyheist

cd ../x86_64-pc-windows-gnu/release
zip -r "../../windows-${version[1]}.zip" honeyheist.exe
