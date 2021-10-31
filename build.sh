#!/bin/bash


PROJECT_DIR=$PWD;

rm -rf ./dist 2>/dev/null
mkdir -p ./dist

# in webapp
cd ./webapp/
npm install && npm run build
cp -dpr dist/ $PROJECT_DIR/dist/www

#back to root folder
cd $PROJECT_DIR

#in cli
cargo build --manifest-path=cli/Cargo.toml --release && cp cli/target/release/la-poste-releve-cli ./dist/lpr-rs
cp -dpr cli/data/*.csv $PROJECT_DIR/dist
cp cli/config.toml $PROJECT_DIR/dist/config.toml
cp cli/init-db.toml $PROJECT_DIR/dist/init-db.toml
