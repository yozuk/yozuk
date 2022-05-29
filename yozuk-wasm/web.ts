import init, { exec, push_stream } from './wasm-web/yozuk_wasm'
import { Base64 } from "js-base64";
import { Result, Output } from './output'

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
        const result = JSON.parse(exec(command, JSON.stringify(i18n)));
        if (result.outputs) {
            result.outputs.forEach((output) => {
                output.blocks.forEach((block) => {
                    const { data } = block;
                    if (data) {
                        block.data = Base64.decode(data);
                    }
                });
            });
        }
        return result;
    }
}
