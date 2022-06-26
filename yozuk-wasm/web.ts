import init, { exec, random_suggests, push_stream } from './wasm-web/yozuk_wasm'
import { decode } from 'base64-arraybuffer';
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
        const textDecoder = new TextDecoder('utf-8', { fatal: true });
        if (result.outputs) {
            result.outputs.forEach((output) => {
                output.blocks.forEach((block) => {
                    const { data } = block;
                    if (data) {
                        const decoded = decode(data);
                        try {
                            block.data = textDecoder.decode(decoded);
                        } catch {
                            block.data = decoded;
                        }
                    }
                });
            });
        }
        return result;
    }

    async random_suggests(amount: number = 5): Promise<String[]> {
        await init_once();
        return JSON.parse(random_suggests(amount));
    }
}
