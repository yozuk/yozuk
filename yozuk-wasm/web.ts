import init, { exec, push_stream } from './wasm-web/yozuk_wasm'

let initialized: boolean = false;

async function init_once() {
    if (!initialized) {
        await init()
        initialized = true;
    }
}

export class Yozuk {
    async exec(command: string, i18n: I18n = {}, streams: Uint8Array[] = []): Promise<any> {
        await init_once();
        for (const stream of streams) {
            push_stream(stream);
        }
        return JSON.parse(exec(command, JSON.stringify(i18n)));
    }
}

interface I18n {
    locale?: string
    timezone?: string
    location?: [number, number]
}