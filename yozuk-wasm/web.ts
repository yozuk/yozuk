import init, { exec, push_stream } from './wasm-web/yozuk_wasm'
import { Result } from './output'

let initialized: boolean = false;

async function init_once() {
    if (!initialized) {
        await init()
        initialized = true;
    }
}

export class Yozuk {
    async exec(command: string, streams: Uint8Array[] = []): Promise<Result> {
        await init_once();
        const i18n = {
            locale: navigator.language,
            timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
        };
        for (const stream of streams) {
            push_stream(stream);
        }
        return JSON.parse(exec(command, JSON.stringify(i18n)));
    }
}
