const { execSync } = require('node:child_process');

const options = {
    env: {
        "RUSTFLAGS": "-C opt-level=z"
    }
}

execSync('wasm-pack build -d wasm-web -t web --release', { stdio: 'inherit' });
