import init, { exec, random_suggests, push_stream } from './wasm-web/yozuk_wasm'
import { YozukBase, I18n } from './yozuk'

let initialized: boolean = false;

async function init_once() {
    if (!initialized) {
        await init()
        initialized = true;
    }
}

export class Yozuk extends YozukBase {
    exec_impl(command: string, i18n: string): Promise<string> {
        return init_once().then(() => exec(command, i18n));
    }

    push_stream_impl(stream: Uint8Array): Promise<void> {
        return init_once().then(() => push_stream(stream));
    }

    random_suggests_impl(amount: number): Promise<string> {
        return init_once().then(() => random_suggests(amount));
    }

    i18n(): I18n {
        return {
            locale: navigator.language,
            timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
        };
    }
}
