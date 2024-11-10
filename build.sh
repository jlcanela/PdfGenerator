#!/bin/bash

cd PdfGenerator
cargo build --release
cargo test
cd ..

mkdir -p PdfApp/bin/Debug/net8.0
cp PdfGenerator/target/release/libpdf_generator.so PdfApp/bin/Debug/net6.0/
