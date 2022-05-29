#!/usr/bin/node

const { execSync } = require('node:child_process');

const options = {
    env: {
        "RUSTFLAGS": "-C opt-level=z"
    },
    stdio: 'inherit'
};

execSync('wasm-pack build -d wasm-web -t web --release', options);
