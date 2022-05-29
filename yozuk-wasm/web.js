import init, { exec, push_stream } from './wasm-web/yozuk_wasm'

let initialized = false;

async function init_once() {
    if (!initialized) {
        await init()
        initialized = true;
    }
}

export class Yozuk {
    async exec() {
        await init_once();
    }
}