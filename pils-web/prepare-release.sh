#!/bin/sh

wasm-pack build --target web --release

echo "paving over old docs dir"
rm -rf ../docs

mkdir ../docs

echo "removing pkg/.gitignore --- fite me"
rm pkg/.gitignore

echo "moving over wasm artifacts"
mv pkg ../docs/

echo "copying over assets"
cp index.html ../docs/
cp main.js ../docs/
cp material_pils.svg ../docs/
cp style.css ../docs/
cp favicon.ico ../docs/
