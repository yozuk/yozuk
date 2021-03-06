#!/usr/bin/node

const { execSync } = require('node:child_process');

function build(out, target, profile, env = {}) {
    const dir = `${__dirname}/../yozuk-wasm`;
    const options = {
        cwd: dir,
        env: {
            ...process.env,
            ...env,
            "RUSTFLAGS": "-C opt-level=z"
        },
        stdio: 'inherit'
    };
    execSync(`wasm-pack build -d ${out} -t ${target} --${profile}`, options);
    execSync(`tar -C ${dir} -zcvf yozuk-${out}.tar.gz ${out}`);
}

build('wasm-web', 'web', 'release', { "RUSTFLAGS": "-C opt-level=z" });
build('wasm-web-debug', 'web', 'debug');
build('wasm-node', 'nodejs', 'release', { "RUSTFLAGS": "-C opt-level=z" });
build('wasm-node-debug', 'nodejs', 'debug');
